#![allow(dead_code)]

extern crate variant_count;
extern crate field_types;

use variant_count::VariantCount;
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
#[field_type_derive(VariantCount, Debug, Clone, PartialEq)]
struct TestGen<'a, T: 'a, U>
    where U: 'a
{
    first: T,
    second_field: Option<&'a U>,
    #[field_type(skip)]
    third: &'a T,
    #[field_types = "skip"]
    fourth: U,
}

#[derive(FieldType)]
#[field_types_derive(VariantCount, Debug, Clone, PartialEq)]
struct TestTypesDerive {
    first: i32,
    second: bool,
}

#[derive(FieldType)]
#[field_type_derive(VariantCount, Debug, Clone, PartialEq)]
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

    let _field = TestGenFieldType::First::<_, &str>(2);
    let field = TestGenFieldType::SecondField::<i32, &str>(None);
    match field {
        TestGenFieldType::First(_) => (),
        TestGenFieldType::SecondField(_) => (),
    }

    let first_field = TestTypesDeriveFieldType::First(2);
    match first_field {
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
    let first_field = TestGenFieldType::First::<_, &str>(2);
    let second_field = TestGenFieldType::SecondField::<i32, &str>(None);
    assert_ne!(first_field, second_field);
    assert_eq!(first_field, first_field.clone());
    assert_eq!("First(2)", format!("{:?}", first_field));

    let field = TestTypesDeriveFieldType::First(1).clone();
    assert_eq!(TestTypesDeriveFieldType::First(1), field);
    assert_ne!(TestTypesDeriveFieldType::First(2), field);
    assert_eq!("First(1)", format!("{:?}", field));

    let field = TestTypesDeriveFieldType::Second(true).clone();
    assert_eq!(TestTypesDeriveFieldType::Second(true), field);
    assert_ne!(TestTypesDeriveFieldType::Second(false), field);
    assert_eq!("Second(true)", format!("{:?}", field));

    let field = TestTypeDeriveFieldType::First(1).clone();
    assert_eq!(TestTypeDeriveFieldType::First(1), field);
    assert_ne!(TestTypeDeriveFieldType::First(2), field);
    assert_eq!("First(1)", format!("{:?}", field));

    let field = TestTypeDeriveFieldType::Second(true).clone();
    assert_eq!(TestTypeDeriveFieldType::Second(true), field);
    assert_ne!(TestTypeDeriveFieldType::Second(false), field);
    assert_eq!("Second(true)", format!("{:?}", field));
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

    let message = "test".to_string();
    let test = TestGen {
        first: 1,
        second_field: Some(&message),
        third: &2,
        fourth: message.clone(),
    };
    let fields: [TestGenFieldType<i32, String>; TestGenFieldType::<i32, String>::VARIANT_COUNT] = test.into();
    assert!(match fields {
        [TestGenFieldType::First(1), TestGenFieldType::SecondField(Some(s))] if s == &message => true,
        _ => false,
    });

    let test = TestTypesDerive {
        first: 1,
        second: true,
    };
    let fields: [TestTypesDeriveFieldType; TestTypesDeriveFieldType::VARIANT_COUNT] = test.into();
    assert_eq!(TestTypesDeriveFieldType::First(1), fields[0]);
    assert_eq!(TestTypesDeriveFieldType::Second(true), fields[1]);
}
