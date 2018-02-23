#[macro_use]
extern crate virtual_view;

use virtual_view::{Event, Prop, Props};

#[test]
fn test_prop_null() {
    assert_eq!(prop!(null), Prop::Null);
}

#[test]
fn test_prop_bool() {
    assert_eq!(prop!(true), Prop::Boolean(true));
    assert_eq!(prop!(false), Prop::Boolean(false));
}

#[test]
fn test_prop_number() {
    assert_eq!(prop!(0), Prop::Number(0.0));
    assert_eq!(prop!(1.0), Prop::Number(1.0));
    assert_eq!(prop!(-1.0), Prop::Number(-1.0));
}

#[test]
fn test_prop_string() {
    assert_eq!(prop!("Hello, world!"), Prop::String("Hello, world!".into()));
}

#[test]
fn test_prop_array() {
    assert_eq!(prop!([0, 1]), Prop::Array(vec![0, 1].into()));
}

#[test]
fn test_prop_map() {
    let mut props = Props::new();
    props.insert("key", "value");
    assert_eq!(prop!({"key": "value"}), Prop::Object(props));
}

#[test]
fn test_prop_full() {
    let copy = 0.0;

    let f = props! {
        "count": 0,
        "onclick": move |e: &mut Event| {
            let _ = copy;
            let _ = e;
        },
        "array": [0, 1, 2]
    };

    assert!(f.get("onclick").is_event());
}

#[test]
fn test_prop_extend() {
    let a = props! {
        "0": 0,
    };
    let b = props! {
        "1": 1,
        ... a
    };

    assert_eq!(b.get("0"), &Prop::Number(0.0));
    assert_eq!(b.get("1"), &Prop::Number(1.0));
}
