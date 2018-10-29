#![allow(dead_code)]

extern crate field_types;

use field_types::FieldType;

#[derive(FieldType)]
struct Test {
    first: i32,
    second_field: Option<String>,
    #[field_type(skip)]
    third: bool,
    #[field_type = "skip"]
    fourth: bool,
}

#[derive(FieldType)]
#[field_types_derive(Debug, Clone, PartialEq)]
struct TestTypesDerive {
    first: i32,
    second: bool,
}

#[derive(FieldType)]
#[field_type_derive(Debug, Clone, PartialEq)]
struct TestTypeDerive {
    first: i32,
    second: bool,
}

#[test]
fn full_field_type_variants() {
    let _field = TestFieldType::First(2);
    let field = TestFieldType::SecondField(None);
    match field {
        TestFieldType::First(_) => (),
        TestFieldType::SecondField(_) => (),
    }

    let _field = TestTypesDeriveFieldType::First(2);
    let field = TestTypesDeriveFieldType::Second(false);
    match field {
        TestTypesDeriveFieldType::First(_) => (),
        TestTypesDeriveFieldType::Second(_) => (),
    }

    let _field = TestTypeDeriveFieldType::First(2);
    let field = TestTypeDeriveFieldType::Second(false);
    match field {
        TestTypeDeriveFieldType::First(_) => (),
        TestTypeDeriveFieldType::Second(_) => (),
    }
}

#[test]
fn derive_field_type() {
    let field = TestTypesDeriveFieldType::First(1).clone();
    assert_eq!(TestTypesDeriveFieldType::First(1), field);
    assert_ne!(TestTypesDeriveFieldType::First(2), field);

    let field = TestTypesDeriveFieldType::Second(true).clone();
    assert_eq!(TestTypesDeriveFieldType::Second(true), field);
    assert_ne!(TestTypesDeriveFieldType::Second(false), field);
}

#[test]
fn into_field_type() {
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

    let test = TestTypesDerive {
        first: 1,
        second: true,
    };
    let fields: [TestTypesDeriveFieldType; 2] = test.into();
    assert_eq!(TestTypesDeriveFieldType::First(1), fields[0]);
    assert_eq!(TestTypesDeriveFieldType::Second(true), fields[1]);
}
