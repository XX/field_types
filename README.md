# Field Types

[![Crates.io](https://img.shields.io/crates/v/field_types.svg)](https://crates.io/crates/field_types)
[![Docs](https://docs.rs/field_types/badge.svg)](https://docs.rs/field_types)

This crate provides `FieldName` and `FieldType` derive macros for deriving `StructFieldName` and `StructFieldType` enums for any struct `Struct` with some fields.

The `..FieldName` enum contains unit types with names corresponding to the names of the structure fields.
Additionally, you can get static string representation of a field name with `name` method and get `..FieldName` variant by string with `by_name` method.

The `FieldName` usage example:

```rust
use field_types::FieldName;

#[derive(FieldName)]
struct Test {
    first: i32,
    second_field: Option<String>,
    #[field_name(skip)]
    third: bool,
}

assert_eq!(TestFieldName::First.name(), "first");
assert_eq!(TestFieldName::SecondField.name(), "second_field");

assert_eq!(Some(TestFieldName::First), TestFieldName::by_name("first"));
assert_eq!(Some(TestFieldName::SecondField), TestFieldName::by_name("second_field"));
assert_eq!(None, TestFieldName::by_name("third"));
```

The `..FieldType` enum contains some types with names corresponding to the names of the structure fields and
with values corresponding to the value types of the structure fields.

The `FieldType` usage example:

```rust
use field_types::FieldType;
use variant_count::VariantCount;

#[derive(FieldType)]
#[field_type_derive(VariantCount)]
struct Test {
    first: i32,
    second_field: Option<String>,
    #[field_type(skip)]
    third: bool,
}

let test = Test {
    first: 1,
    second_field: Some("test".to_string()),
    third: true,
};
let fields: [TestFieldType; TestFieldType::VARIANT_COUNT] = test.into();
assert!(match fields {
    [TestFieldType::First(1), TestFieldType::SecondField(Some(ref s))] if s == "test" => true,
    _ => false,
});
```

In both cases you can skip fields with `#[attr(skip)]` or `#[attr = "skip"]` field attributes, where `attr` is `field_name` for `FieldName`, `field_type` for `FieldType` or `field_types` for any field type derives.
You can also specifying some derives for generated enums with `#[attr_derive(..)]` structure attribute, where `attr_derive` is `field_name_derive`, `field_type_derive` or `field_types_derive`. For example:

```rust
#[derive(FieldType, FieldName)]
#[field_types_derive(VariantCount, Debug, Clone, PartialEq)]
struct Test {
    first: i32,
    second: Option<String>,
    #[field_types(skip)]
    third: bool,
    #[field_name = "skip"]
    fourth: bool,
}
```

By default, `FieldName` has derive `Debug`, `PartialEq`, `Eq`, `Clone` and `Copy`. More usage examples see in [tests](tests) directory.

## Usage

If you're using Cargo, just add it to your Cargo.toml:

```toml
[dependencies]
field_types = "*"
```

## License

MIT