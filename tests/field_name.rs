#![allow(dead_code)]

#[macro_use]
extern crate field_enums;

#[derive(FieldName)]
struct Test {
    first: i32,
    second_field: Option<String>,
    #[field_name(skip)]
    third: bool,
    #[field_name = "skip"]
    fourth: bool,
}

#[derive(FieldName)]
#[field_enums_derive(Debug, Clone, PartialEq)]
struct EnumsDerive {
    first: i32,
    second: bool,
}

#[derive(FieldName)]
#[field_name_derive(Debug, Clone, PartialEq)]
struct NameDerive {
    first: i32,
    second: bool,
}

#[test]
fn full_field_name_variants() {
    let _field = TestFieldName::First;
    let field = TestFieldName::SecondField;
    match field {
        TestFieldName::First => (),
        TestFieldName::SecondField => (),
    }

    let _field = EnumsDeriveFieldName::First;
    let field = EnumsDeriveFieldName::Second;
    match field {
        EnumsDeriveFieldName::First => (),
        EnumsDeriveFieldName::Second => (),
    }

    let _field = NameDeriveFieldName::First;
    let field = NameDeriveFieldName::Second;
    match field {
        NameDeriveFieldName::First => (),
        NameDeriveFieldName::Second => (),
    }
}

#[test]
fn derive_field_name() {
    let name = TestFieldName::First;
    assert_eq!(TestFieldName::First, name);
    assert_ne!(TestFieldName::SecondField, name);

    let name = EnumsDeriveFieldName::First.clone();
    assert_eq!(EnumsDeriveFieldName::First, name);
    assert_ne!(EnumsDeriveFieldName::Second, name);

    let name = EnumsDeriveFieldName::Second.clone();
    assert_eq!(EnumsDeriveFieldName::Second, name);
    assert_ne!(EnumsDeriveFieldName::First, name);
}

#[test]
fn into_field_name() {
    let test = Test {
        first: 1,
        second_field: Some("test".to_string()),
        third: true,
        fourth: true,
    };
    let fields: [TestFieldName; 2] = (&test).into();
    assert!(match fields {
        [TestFieldName::First, TestFieldName::SecondField] => true,
        _ => false,
    });

    let test = EnumsDerive {
        first: 1,
        second: true,
    };
    let fields: [EnumsDeriveFieldName; 2] = (&test).into();
    assert_eq!(EnumsDeriveFieldName::First, fields[0]);
    assert_eq!(EnumsDeriveFieldName::Second, fields[1]);
}

#[test]
fn field_name_str() {
    assert_eq!(TestFieldName::First.name(), "first");
    assert_eq!(TestFieldName::SecondField.name(), "second_field");

    assert_eq!(Some(TestFieldName::First), TestFieldName::by_name("first"));
    assert_eq!(Some(TestFieldName::SecondField), TestFieldName::by_name("second_field"));
    assert_eq!(None, TestFieldName::by_name("third"));
}
