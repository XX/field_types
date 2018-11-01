/*!
This crate provides `FieldName` and `FieldType` derive macros for deriving enums, corresponding to the fields of structs.

## Features

* `..FieldName` enum
    * Variants with UpperCamelCase unit type names corresponding to the snake_case field names of the struct
    * Skipping fields with `#[field_name(skip)]` or `#[field_types(skip)]` attributes
    * Specifying some derives for generated enums with `#[field_name_derive(..)]` or `#[field_types_derive(..)]` structure attributes.
By default, `..FieldName` has derive `Debug`, `PartialEq`, `Eq`, `Clone` and `Copy`.
    * `From`/`Into` convert the struct reference to an array of variants
    * `name`/`by_name` methods for convert enum variants to/from string representation field names
* `..FieldType` enum
    * Variants with UpperCamelCase type names corresponding to the snake_case field names of the struct
and with values corresponding to the value types of the struct fields
    * Skipping fields with `#[field_type(skip)]` or `#[field_types(skip)]` attributes
    * Specifying some derives for generated enums with `#[field_type_derive(..)]` or `#[field_types_derive(..)]` structure attributes
    * `Into` convert the struct into an array of variants with field values

## Example

```rust
extern crate field_types;

use field_types::{FieldName, FieldType};

#[derive(FieldName, FieldType)]
struct Test {
    first: i32,
    second_field: Option<String>,
    #[field_types(skip)]
    third: bool,
}

fn main() {
    assert_eq!(TestFieldName::First.name(), "first");
    assert_eq!(TestFieldName::SecondField.name(), "second_field");

    assert_eq!(Some(TestFieldName::First), TestFieldName::by_name("first"));
    assert_eq!(Some(TestFieldName::SecondField), TestFieldName::by_name("second_field"));
    assert_eq!(None, TestFieldName::by_name("third"));

    let test = Test {
        first: 1,
        second_field: Some("test".to_string()),
        third: true,
    };
    let fields: [TestFieldType; 2] = test.into();
    assert!(match fields {
        [TestFieldType::First(1), TestFieldType::SecondField(Some(ref s))] if s == "test" => true,
        _ => false,
    });
}
```

## Usage

If you're using Cargo, just add it to your Cargo.toml:

```toml
[dependencies]
field_types = "*"
```

Use `FieldName` and/or `FieldType` in `derive` struct attribute.
!*/

extern crate proc_macro;
extern crate syn;
extern crate quote;
extern crate heck;

use std::iter::FromIterator;
use proc_macro::TokenStream;
use syn::{
    DeriveInput, Ident, Type, Generics, Attribute, Fields, Meta,
    export::{Span, TokenStream2}
};
use quote::{quote, ToTokens};
use heck::CamelCase;

#[proc_macro_derive(FieldType, attributes(field_types, field_type, field_types_derive, field_type_derive))]
pub fn field_type(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (vis, ty, generics) = (&ast.vis, &ast.ident, &ast.generics);
    let enum_ty = Ident::new(&(ty.to_string() + "FieldType"), Span::call_site());
    let derive = get_enum_derive(&ast.attrs, &["field_types_derive", "field_type_derive"], quote! {});

    let fields = filter_fields(match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("FieldType can only be derived for structures"),
    }, "field_type");

    if fields.is_empty() {
        panic!("FieldType can only be derived for non-empty structures");
    }

    let converter = get_type_converter(ty, &generics, &enum_ty, &fields);

    let field_type_variants = fields.iter()
        .map(|(_, field_ty, variant_ident)| {
            quote! {
                #variant_ident(#field_ty)
            }
        });

    let where_clause = generics.where_clause.as_ref();
    let tokens = quote! {
        #derive
        #vis enum #enum_ty #generics #where_clause {
            #(#field_type_variants),*
        }

        #converter
    };
    tokens.into()
}

#[proc_macro_derive(FieldName, attributes(field_types, field_name, field_types_derive, field_name_derive))]
pub fn field_name(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (vis, ty, generics) = (&ast.vis, &ast.ident, &ast.generics);
    let enum_ty = Ident::new(&(ty.to_string() + "FieldName"), Span::call_site());
    let derive = get_enum_derive(&ast.attrs, &["field_types_derive", "field_name_derive"],
                            quote! { #[derive(Debug, PartialEq, Eq, Clone, Copy)] });

    let fields = filter_fields(match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("FieldName can only be derived for structures"),
    }, "field_name");

    if fields.is_empty() {
        panic!("FieldName can only be derived for non-empty structures");
    }

    let field_name_variants = fields.iter()
        .map(|(_, _, variant_ident)| {
            quote! {
                #variant_ident
            }
        });

    let field_name_to_strs = fields.iter()
        .map(|(field_ident, _, variant_ident)| {
            let field_name = field_ident.to_string();
            quote! {
                #enum_ty::#variant_ident => #field_name
            }
        });

    let field_name_by_strs = fields.iter()
        .map(|(_, _, variant_ident)| {
            quote! {
                if #enum_ty::#variant_ident.name() == name { return Some(#enum_ty::#variant_ident) }
            }
        });

    let field_name_constructs = fields.iter()
        .map(|(_, _, variant_ident)| {
            quote! {
                #enum_ty::#variant_ident
            }
        });

    let fields_count = fields.len();

    let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();
    let from_lifetime = quote! { 'field_name_from_lifetime__ };

    let mut impl_generics_tokens = TokenStream2::new();
    impl_generics.to_tokens(&mut impl_generics_tokens);
    if impl_generics_tokens.is_empty() {
        impl_generics_tokens = quote! { <#from_lifetime> };
    } else {
        let mut tokens: Vec<_> = quote! { #from_lifetime, }.into_iter().collect();
        let mut gen_iter = impl_generics_tokens.into_iter();
        if let Some(token) = gen_iter.next() {
            tokens.insert(0, token);
        }
        tokens.extend(gen_iter);
        impl_generics_tokens = TokenStream2::from_iter(tokens);
    }

    let tokens = quote! {
        #derive
        #vis enum #enum_ty {
            #(#field_name_variants),*
        }

        impl #enum_ty {
            #vis fn name(&self) -> &'static str {
                match *self {
                    #(#field_name_to_strs),*
                }
            }

            #vis fn by_name(name: &str) -> Option<Self> {
                #(#field_name_by_strs)*
                None
            }
        }

        impl #impl_generics_tokens From<& #from_lifetime #ty #ty_generics> for [#enum_ty; #fields_count] {
            fn from(_source: & #from_lifetime #ty #ty_generics) -> Self {
                [#(#field_name_constructs),*]
            }
        }
    };
    tokens.into()
}

fn get_enum_derive(attrs: &[Attribute], derive_attr_names: &[&str], default: TokenStream2) -> TokenStream2 {
    attrs.iter()
        .filter_map(|attr| attr.interpret_meta()
            .and_then(|meta| {
                for attr_name in derive_attr_names {
                    if meta.name() == attr_name {
                        if let Meta::List(mut meta_list) = meta {
                            meta_list.ident = Ident::new("derive", Span::call_site());
                            return Some(meta_list);
                        }
                    }
                }
                None
            })
        )
        .next()
        .map(|meta_list| quote! { #[#meta_list] })
        .unwrap_or(default)
}

fn get_type_converter(ty: &Ident, generics: &Generics, enum_ty: &Ident, fields: &Vec<(Ident, Type, Ident)>) -> TokenStream2 {
    let fields_idents = fields.iter()
        .map(|(field_ident, _, _)| {
            quote! {
                #field_ident
            }
        });

    let field_type_constructs = fields.iter()
        .map(|(field_ident, _, variant_ident)| {
            quote! {
                #enum_ty::#variant_ident(#field_ident)
            }
        });

    let fields_count = fields.len();
    if generics.params.is_empty() {
        quote! {
            impl From<#ty> for [#enum_ty; #fields_count] {
                fn from(source: #ty) -> Self {
                    let #ty { #(#fields_idents,)* .. } = source;
                    [#(#field_type_constructs),*]
                }
            }
        }
    } else {
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        quote! {
            impl #impl_generics Into<[#enum_ty #ty_generics; #fields_count]> for #ty #ty_generics
                #where_clause
            {
                fn into(self) -> [#enum_ty #ty_generics; #fields_count] {
                    let #ty { #(#fields_idents,)* .. } = self;
                    [#(#field_type_constructs),*]
                }
            }
        }
    }
}

fn filter_fields(fields: &Fields, skip_attr_name: &str) -> Vec<(Ident, Type, Ident)> {
    fields.iter()
        .filter_map(|field| {
            if field.attrs.iter()
                .find(|attr| has_skip_attr(attr, &["field_types", skip_attr_name]))
                .is_none() && field.ident.is_some()
            {
                let field_ty = field.ty.clone();
                let field_ident = field.ident.as_ref().unwrap().clone();
                let field_name = field.ident.as_ref().unwrap().to_string();
                let variant_ident = Ident::new(&field_name.to_camel_case(), Span::call_site());
                Some((field_ident, field_ty, variant_ident))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn has_skip_attr(attr: &Attribute, attr_names: &[&str]) -> bool {
    attr.interpret_meta()
        .and_then(|meta| {
            for attr_name in attr_names {
                if meta.name() == attr_name {
                    return Some(meta);
                }
            }
            None
        })
        .map(|meta| {
            let value = match meta {
                Meta::List(ref list) => list.nested.first()
                    .expect("Attribute value can't be empty")
                    .into_value()
                    .clone()
                    .into_token_stream()
                    .to_string(),

                Meta::NameValue(ref name_value) => name_value.lit
                    .clone()
                    .into_token_stream()
                    .to_string(),

                _ => panic!("Unknown attribute value, only `skip` allowed."),
            };
            if value != "skip" && value.find("\"skip\"").is_none() {
                panic!("Unknown attribute value `{}`, only `skip` allowed.", value);
            }
        })
        .is_some()
}
