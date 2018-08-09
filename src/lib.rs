extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate heck;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use syn::{DeriveInput, Ident, Type, Attribute, Fields};
use heck::CamelCase;

#[proc_macro_derive(FieldType, attributes(field_enums, field_type))]
pub fn field_type(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let ty = &ast.ident;
    let vis = &ast.vis;
    let enum_ty = Ident::new(&(ty.to_string() + "FieldType"), Span::call_site());

    let fields = filter_fields(match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("FieldType can only be derived for structures"),
    }, "field_type");

    let fields_count = fields.len();
    if fields_count == 0 {
        panic!("FieldType can only be derived for non-empty structures");
    }

    let fields_idents = fields.iter()
        .map(|(field_ident, _, _)| {
            quote! {
                #field_ident
            }
        });

    let field_type_variants = fields.iter()
        .map(|(_, field_ty, variant_ident)| {
            quote! {
                #variant_ident(#field_ty)
            }
        });

    let field_type_constructs = fields.iter()
        .map(|(field_ident, _, variant_ident)| {
            quote! {
                #enum_ty::#variant_ident(#field_ident)
            }
        });

    let tokens = quote! {
        #vis enum #enum_ty {
            #(#field_type_variants),*
        }

        impl From<#ty> for [#enum_ty; #fields_count] {
            fn from(source: #ty) -> Self {
                let #ty { #(#fields_idents,)* .. } = source;
                [#(#field_type_constructs),*]
            }
        }
    };
    tokens.into()
}

#[proc_macro_derive(FieldName, attributes(field_enums, field_name))]
pub fn field_name(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let ty = &ast.ident;
    let vis = &ast.vis;
    let enum_ty = Ident::new(&(ty.to_string() + "FieldName"), Span::call_site());

    let fields = filter_fields(match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("FieldName can only be derived for structures"),
    }, "field_name");

    let fields_count = fields.len();
    if fields_count == 0 {
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

    let tokens = quote! {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

        impl<'a> From<&'a #ty> for [#enum_ty; #fields_count] {
            fn from(_source: &'a #ty) -> Self {
                [#(#field_name_constructs),*]
            }
        }
    };
    tokens.into()
}

fn filter_fields(fields: &Fields, skip_attr_name: &str) -> Vec<(Ident, Type, Ident)> {
    fields.iter()
        .filter_map(|field| {
            if field.attrs.iter()
                .find(|attr| has_skip_attr(attr, &["field_enums", skip_attr_name]))
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
    if attr.path.segments.iter()
        .find(|segment| {
            let name = &segment.ident.to_string();
            for attr_name in attr_names {
                if name == attr_name {
                    return true;
                }
            }
            false
        }).is_some()
    {
        attr.tts.clone().into_iter()
            .next()
            .map(|tt| check_skip(&tt))
            .unwrap_or(false)
    } else {
        false
    }
}

fn check_skip(tt: &TokenTree) -> bool {
    match tt {
        TokenTree::Ident(i) if i == "skip" => true,
        TokenTree::Ident(i) => panic!("Unknown attribute identifier `{}`", i),
        TokenTree::Group(g) => g.stream().into_iter().next()
            .map(|tt| check_skip(&tt))
            .unwrap_or(false),
        _ => false,
    }
}
