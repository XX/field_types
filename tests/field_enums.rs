#![allow(dead_code)]

#[macro_use]
extern crate field_enums;

#[derive(FieldType, FieldName)]
#[field_enums_derive(Debug, Clone, PartialEq)]
struct Test {
    first: i32,
    second_field: Option<String>,
    #[field_enums(skip)]
    third: bool,
    #[field_enums = "skip"]
    fourth: bool,
}

#[test]
fn full_field_enums_variants() {
    let _field = TestFieldType::First(2);
    let field = TestFieldType::SecondField(None);
    match field {
        TestFieldType::First(_) => (),
        TestFieldType::SecondField(_) => (),
    }

    let _field = TestFieldName::First;
    let field = TestFieldName::SecondField;
    match field {
        TestFieldName::First => (),
        TestFieldName::SecondField => (),
    }
}

#[test]
fn derive_field_enums() {
    let field = TestFieldType::First(1).clone();
    assert_eq!(TestFieldType::First(1), field);
    assert_ne!(TestFieldType::First(2), field);

    let field = TestFieldType::SecondField(Some("test".to_string())).clone();
    assert_eq!(TestFieldType::SecondField(Some("test".to_string())), field);
    assert_ne!(TestFieldType::SecondField(Some("".to_string())), field);

    let name = TestFieldName::First;
    assert_eq!(TestFieldName::First, name);
    assert_ne!(TestFieldName::SecondField, name);

    let name = TestFieldName::SecondField;
    assert_eq!(TestFieldName::SecondField, name);
    assert_ne!(TestFieldName::First, name);
}

#[test]
fn into_field_enums() {
    let test = Test {
        first: 1,
        second_field: Some("test".to_string()),
        third: true,
        fourth: true,
    };
    let fields: [TestFieldType; 2] = test.into();
    assert!(match fields {
        [TestFieldType::First(1), TestFieldType::SecondField(Some(ref s))] if s == "test" => true,
        _ => false,
    });

    let test = Test {
        first: 1,
        second_field: Some("test".to_string()),
        third: true,
        fourth: true,
    };
    let names: [TestFieldName; 2] = (&test).into();
    assert!(match names {
        [TestFieldName::First, TestFieldName::SecondField] => true,
        _ => false,
    });
}

#[test]
fn field_name_str() {
    assert_eq!(TestFieldName::First.name(), "first");
    assert_eq!(TestFieldName::SecondField.name(), "second_field");

    assert_eq!(Some(TestFieldName::First), TestFieldName::by_name("first"));
    assert_eq!(Some(TestFieldName::SecondField), TestFieldName::by_name("second_field"));
    assert_eq!(None, TestFieldName::by_name("third"));
}
