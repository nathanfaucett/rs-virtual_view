#[macro_use]
extern crate view;

use view::diff_props;

#[test]
fn test_diff_props() {
    let a = prop!({"a": "old"});
    let b = prop!({"a": "new"});
    assert_eq!(diff_props(&a, &b), prop!({"a": "new"}));
    assert_eq!(diff_props(&a, &a), prop!(null));

    let a = prop!({ "a": { "a": [] } });
    let b = prop!({ "a": { "a": [1, 2] } });
    assert_eq!(diff_props(&a, &b), prop!({ "a": { "a": [1, 2] } }));

    let a = prop!({ "a": 0, "b": 1 });
    let b = prop!({ "a": 0 });
    assert_eq!(diff_props(&a, &b), prop!({ "b": null }));
    assert_eq!(diff_props(&b, &a), prop!({ "b": 1 }));
}
