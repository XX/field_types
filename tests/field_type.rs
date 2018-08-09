#[macro_use]
extern crate field_enums;

#[allow(dead_code)]

#[derive(FieldType)]
struct Test {
    first: i32,
    second_field: Option<String>,
    #[field_type(skip)]
    third: bool,
}

#[test]
fn field_type_variants() {
    let _field = TestFieldType::First(2);
    let field = TestFieldType::SecondField(None);
    match field {
        TestFieldType::First(_) => (),
        TestFieldType::SecondField(_) => (),
    }
}

#[test]
fn into_field_type() {
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
}
