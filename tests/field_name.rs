#[macro_use]
extern crate field_enums;

#[allow(dead_code)]

#[derive(FieldName)]
struct Test {
    first: i32,
    second_field: Option<String>,
    #[field_name(skip)]
    third: bool,
}

#[test]
fn field_name_variants() {
    let _field = TestFieldName::First;
    let field = TestFieldName::SecondField;
    match field {
        TestFieldName::First => (),
        TestFieldName::SecondField => (),
    }
}

#[test]
fn into_field_name() {
    let test = Test {
        first: 1,
        second_field: Some("test".to_string()),
        third: true,
    };
    let fields: [TestFieldName; 2] = (&test).into();
    assert!(match fields {
        [TestFieldName::First, TestFieldName::SecondField] => true,
        _ => false,
    });
}

#[test]
fn field_name_str() {
    assert_eq!(TestFieldName::First.name(), "first");
    assert_eq!(TestFieldName::SecondField.name(), "second_field");

    assert!(if let Some(TestFieldName::First) = TestFieldName::by_name("first") {true} else {false});
    assert!(if let Some(TestFieldName::SecondField) = TestFieldName::by_name("second_field") {true} else {false});
    assert!(if let None = TestFieldName::by_name("third") {true} else {false});
}
