#![allow(dead_code)]

extern crate variant_count;
extern crate field_types;

use variant_count::VariantCount;
use field_types::{FieldType, FieldName};

#[derive(FieldType, FieldName)]
#[field_types_derive(VariantCount, Debug, Clone, PartialEq)]
struct Test {
    first: i32,
    second_field: Option<String>,
    #[field_types(skip)]
    third: bool,
    #[field_name = "skip"]
    fourth: bool,
}

#[derive(FieldType, FieldName)]
#[field_types_derive(VariantCount, Debug, Clone, PartialEq)]
struct TestGen<'a, T: 'a, U>
    where U: 'a
{
    first: T,
    second_field: Option<&'a U>,
    #[field_types(skip)]
    third: &'a T,
    #[field_name = "skip"]
    fourth: U,
}

#[test]
fn full_field_types_variants() {
    let _field = TestFieldType::First(2);
    let _field = TestFieldType::Fourth(false);
    let field = TestFieldType::SecondField(None);
    match field {
        TestFieldType::First(_) => (),
        TestFieldType::SecondField(_) => (),
        TestFieldType::Fourth(_) => (),
    }

    let _field = TestFieldName::First;
    let field = TestFieldName::SecondField;
    match field {
        TestFieldName::First => (),
        TestFieldName::SecondField => (),
    }

    let _field = TestGenFieldType::First::<_, bool>(2);
    let _field = TestGenFieldType::Fourth::<i32, _>(false);
    let field = TestGenFieldType::SecondField::<i32, bool>(None);
    match field {
        TestGenFieldType::First(_) => (),
        TestGenFieldType::SecondField(_) => (),
        TestGenFieldType::Fourth(_) => (),
    }

    let _field = TestGenFieldName::First;
    let field = TestGenFieldName::SecondField;
    match field {
        TestGenFieldName::First => (),
        TestGenFieldName::SecondField => (),
    }
}

#[test]
fn derive_field_types() {
    let field = TestFieldType::First(1).clone();
    assert_eq!(TestFieldType::First(1), field);
    assert_ne!(TestFieldType::First(2), field);

    let first_field = TestGenFieldType::First::<_, &str>(2);
    let second_field = TestGenFieldType::SecondField::<i32, &str>(None);
    assert_ne!(first_field, second_field);
    assert_eq!(first_field, first_field.clone());
    assert_eq!("First(2)", format!("{:?}", first_field));

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
fn into_field_types() {
    let test = Test {
        first: 1,
        second_field: Some("test".to_string()),
        third: true,
        fourth: true,
    };
    let fields: [TestFieldType; TestFieldType::VARIANT_COUNT] = test.into();
    assert!(match fields {
        [TestFieldType::First(1), TestFieldType::SecondField(Some(ref s)), TestFieldType::Fourth(true)] if s == "test" => true,
        _ => false,
    });

    let test = Test {
        first: 1,
        second_field: Some("test".to_string()),
        third: true,
        fourth: true,
    };
    let names: [TestFieldName; TestFieldName::VARIANT_COUNT] = (&test).into();
    assert!(match names {
        [TestFieldName::First, TestFieldName::SecondField] => true,
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
        [TestGenFieldType::First(1), TestGenFieldType::SecondField(Some(s)), TestGenFieldType::Fourth(_)] if s == &message => true,
        _ => false,
    });

    let test = TestGen {
        first: 1,
        second_field: Some(&message),
        third: &2,
        fourth: message.clone(),
    };
    let fields: [TestGenFieldName; TestGenFieldName::VARIANT_COUNT] = (&test).into();
    assert!(match fields {
        [TestGenFieldName::First, TestGenFieldName::SecondField] => true,
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
