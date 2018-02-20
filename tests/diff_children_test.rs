#[macro_use]
extern crate view;

use view::diff_children;

#[test]
fn test_diff_children_complex() {
    let prev = [
        view! { <div key=0/> },
        view! { <div key=1/> },
        view! { <div key=2/> },
    ];
    let next = [view! { <div key=2/> }, view! { <div key=0/> }];

    let diff = diff_children(&prev, &next);

    assert!(diff.children[1].is_none());
}

#[test]
fn test_diff_children_keys() {
    let prev = [view! { <div key=0/> }, view! { <div key=1/> }];
    let next = [view! { <div key=1/> }, view! { <div key=0/> }];

    let diff = diff_children(&prev, &next);

    let &(index, ref key) = &diff.removes[0];
    assert_eq!(index, 1);
    assert_eq!(key, &Some(&String::from("1")));

    let &(ref key, index) = &diff.inserts[0];
    assert_eq!(index, 0);
    assert_eq!(key, &Some(&String::from("1")));
}

#[test]
fn test_diff_children_insert() {
    let prev = [view! { <div/> }];
    let next = [view! { <div/> }, view! { <div/> }];

    let diff = diff_children(&prev, &next);

    assert!(diff.children[0].is_some());
    assert!(diff.children[1].is_some());
}

#[test]
fn test_diff_children_remove() {
    let prev = [view! { <div/> }, view! { <div/> }];
    let next = [view! { <div/> }];

    let diff = diff_children(&prev, &next);

    assert!(diff.children[0].is_some());
    assert!(diff.children[1].is_none());
}
