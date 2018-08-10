#![allow(dead_code)]

#[macro_use]
extern crate field_enums;

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
#[field_enums_derive(Debug, Clone, PartialEq)]
struct EnumsDerive {
    first: i32,
    second: bool,
}

#[derive(FieldType)]
#[field_type_derive(Debug, Clone, PartialEq)]
struct TypeDerive {
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

    let _field = EnumsDeriveFieldType::First(2);
    let field = EnumsDeriveFieldType::Second(false);
    match field {
        EnumsDeriveFieldType::First(_) => (),
        EnumsDeriveFieldType::Second(_) => (),
    }

    let _field = TypeDeriveFieldType::First(2);
    let field = TypeDeriveFieldType::Second(false);
    match field {
        TypeDeriveFieldType::First(_) => (),
        TypeDeriveFieldType::Second(_) => (),
    }
}

#[test]
fn derive_field_type() {
    let field = EnumsDeriveFieldType::First(1).clone();
    assert_eq!(EnumsDeriveFieldType::First(1), field);
    assert_ne!(EnumsDeriveFieldType::First(2), field);

    let field = EnumsDeriveFieldType::Second(true).clone();
    assert_eq!(EnumsDeriveFieldType::Second(true), field);
    assert_ne!(EnumsDeriveFieldType::Second(false), field);
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

    let test = EnumsDerive {
        first: 1,
        second: true,
    };
    let fields: [EnumsDeriveFieldType; 2] = test.into();
    assert_eq!(EnumsDeriveFieldType::First(1), fields[0]);
    assert_eq!(EnumsDeriveFieldType::Second(true), fields[1]);
}
