#[macro_use]
extern crate view;

use view::{Array, Event, Prop, Props};

#[test]
fn test_props_null() {
    assert_eq!(props!(null), Prop::Null);
}

#[test]
fn test_props_bool() {
    assert_eq!(props!(true), Prop::Boolean(true));
    assert_eq!(props!(false), Prop::Boolean(false));
}

#[test]
fn test_props_number() {
    assert_eq!(props!(0), Prop::Number(0.0));
    assert_eq!(props!(1.0), Prop::Number(1.0));
    assert_eq!(props!(-1.0), Prop::Number(-1.0));
}

#[test]
fn test_props_string() {
    assert_eq!(
        props!("Hello, world!"),
        Prop::String("Hello, world!".into())
    );
}

#[test]
fn test_props_array() {
    assert_eq!(props!([0, 1]), Prop::Array(vec![0, 1].into()));
}

#[test]
fn test_props_map() {
    let mut props = Props::new();
    props.insert("key", "value");
    assert_eq!(props!({"key": "value"}), Prop::Map(props));
}

#[test]
fn test_props_full() {
    let copy = 0.0;

    let f = props!({
        "count": 0,
        "onclick": move |e: &mut Event| {
            let _ = copy;
            let _ = e;
        },
        "array": [0, 1, 2]
    });

    assert!(f.map().unwrap().get("onclick").unwrap().is_function());
}
