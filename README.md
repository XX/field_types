# Field Types

`FieldName` usage example:

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

`FieldType` usage example:

```rust
use field_types::FieldType;

#[derive(FieldType)]
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
let fields: [TestFieldType; 2] = test.into();
assert!(match fields {
    [TestFieldType::First(1), TestFieldType::SecondField(Some(ref s))] if s == "test" => true,
    _ => false,
});
```
