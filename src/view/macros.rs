use super::super::{Prop, View};

#[doc(hidden)]
pub use std::any::TypeId;

#[doc(hidden)]
#[inline]
pub fn unwrap(mut stack: Vec<View>) -> View {
    stack
        .pop()
        .expect("exactly one element has to exist in stack!")
}

#[doc(hidden)]
#[inline]
pub fn add_child(stack: &mut Vec<View>, child: View) {
    if let Some(parent) = stack.last_mut() {
        parent
            .children_mut()
            .expect("text view can not have children")
            .push(child);
    } else {
        panic!("no views in stack to add child: {:?}", child);
    }
}

#[doc(hidden)]
#[inline]
pub fn add_prop<K, V>(stack: &mut Vec<View>, key: K, value: V)
where
    K: ToString,
    V: Into<Prop>,
{
    if let Some(parent) = stack.last_mut() {
        parent
            .props_mut()
            .expect("text view can not have props")
            .insert(key, value);
    } else {
        panic!(
            "no views in stack to add prop {:?} = {:?}",
            key.to_string(),
            value.into()
        );
    }
}

#[doc(hidden)]
#[inline]
pub fn set_key<T>(stack: &mut Vec<View>, key: T)
where
    T: ToString,
{
    if let Some(parent) = stack.last_mut() {
        parent.set_key(key.to_string());
    } else {
        panic!("no view in stack to add key = {:?}", key.to_string());
    }
}

#[doc(hidden)]
#[inline]
pub fn extend<K, V, I>(stack: &mut Vec<View>, iter: I)
where
    K: ToString,
    V: Into<Prop>,
    I: IntoIterator<Item = (K, V)>,
{
    if let Some(parent) = stack.last_mut() {
        let props = parent.props_mut().expect("text view can not have props");

        for (k, v) in iter {
            props.insert(k, v);
        }
    } else {
        panic!("no view in stack to extend props with");
    }
}

#[doc(hidden)]
#[inline]
pub fn child_to_parent(stack: &mut Vec<View>, end_tag: Option<&'static str>) {
    if let Some(view) = stack.pop() {
        if let Some(end_tag) = end_tag {
            let start_tag = view.kind().expect("text view can not have children");

            if start_tag != end_tag {
                panic!("wrong closing tag: <{:?}> -> </{:?}>", start_tag, end_tag);
            }
        }
        if !stack.is_empty() {
            stack
                .last_mut()
                .unwrap()
                .children_mut()
                .expect("text view can not have children")
                .push(View::from(view));
        } else {
            stack.push(view);
        }
    } else {
        panic!("redundant closing tag: {:?}", end_tag);
    }
}

#[doc(hidden)]
#[inline]
pub fn child_to_parent_component(stack: &mut Vec<View>, end_type_id: TypeId) {
    if let Some(view) = stack.pop() {
        let start_type_id = view.kind()
            .expect("text view can not have children")
            .type_id();

        if start_type_id != end_type_id {
            panic!(
                "wrong closing component: <{:?}> -> </{:?}>",
                start_type_id, end_type_id
            );
        }

        if !stack.is_empty() {
            stack
                .last_mut()
                .unwrap()
                .children_mut()
                .expect("text view can not have children")
                .push(View::from(view));
        } else {
            stack.push(view);
        }
    } else {
        panic!("redundant closing component: {:?}", end_type_id);
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! view_internal {
    // Start of opening tag
    ($stack:ident (< $start_tag:ident $($tail:tt)*)) => (
        $stack.push($crate::View::new_data(stringify!($start_tag)));
        view_internal! { $stack ($($tail)*) }
    );
    // Start of opening Component
    ($stack:ident (< { $start_component:expr } $($tail:tt)*)) => (
        $stack.push($crate::View::new_component($start_component));
        view_internal! { $stack ($($tail)*) }
    );

    // PATTERN: ... props
    ($stack:ident (... { $value:expr } $($tail:tt)*)) => (
        $crate::view::macros::extend(&mut $stack, $value);
        view_internal! { $stack ($($tail)*) }
    );

    // PATTERN: key = { expression }
    ($stack:ident (key = { $value:expr } $($tail:tt)*)) => (
        $crate::view::macros::set_key(&mut $stack, $value);
        view_internal! { $stack ($($tail)*) }
    );
    ($stack:ident (key = $value:tt $($tail:tt)*)) => (
        view_internal! { $stack (key = { $value } $($tail)*) }
    );

    // Props:
    // PATTERN: prop = expression
    ($stack:ident (($key:expr) ($value:expr) $($tail:tt)*)) => (
        $crate::view::macros::add_prop(&mut $stack, $key, $value);
        view_internal! { $stack ($($tail)*) }
    );
    ($stack:ident (($key:expr) {{ $($props:tt)* }} $($tail:tt)*)) => (
        view_internal! { $stack (($key) ( prop!({ $($props)* }) ) $($tail)*) }
    );
    ($stack:ident (($key:expr) {[ $($array:tt)* ]} $($tail:tt)*)) => (
        view_internal! { $stack (($key) ( prop!([ $($array)* ]) ) $($tail)*) }
    );
    ($stack:ident (($key:expr) { block $value:expr } $($tail:tt)*)) => (
        view_internal! { $stack (($key) ($value) $($tail)*) }
    );
    ($stack:ident (($key:expr) { $value:expr } $($tail:tt)*)) => (
        view_internal! { $stack (($key) ($value) $($tail)*) }
    );
    ($stack:ident (($key:expr) $value:tt $($tail:tt)*)) => (
        view_internal! { $stack (($key) ($value) $($tail)*) }
    );

    ($stack:ident ($key:ident = $($tail:tt)*)) => (
        view_internal! { $stack ((stringify!($key)) $($tail)*) }
    );
    ($stack:ident ($key:tt = $($tail:tt)*)) => (
        view_internal! { $stack (($key) $($tail)*) }
    );

    // PATTERN: { each expression }
    ($stack:ident ({ each $eval:expr } $($tail:tt)*)) => (
        let views = $eval;
        for view in views {
            $crate::view::macros::add_child(&mut $stack, view.into());
        }
        view_internal! { $stack ($($tail)*) }
    );
    // PATTERN: { expression }
    ($stack:ident ({ $eval:expr } $($tail:tt)*)) => (
        let view = $crate::View::from($eval);
        $crate::view::macros::add_child(&mut $stack, view);
        view_internal! { $stack ($($tail)*) }
    );
    // End of opening tag
    ($stack:ident (> $($tail:tt)*)) => (
        view_internal! { $stack ($($tail)*) }
    );
    // self closing of tag
    ($stack:ident (/ > $($tail:tt)*)) => (
        $crate::view::macros::child_to_parent(&mut $stack, None);
        view_internal! { $stack ($($tail)*) }
    );
    // tag closing
    ($stack:ident (< / $end_tag:ident > $($tail:tt)*)) => (
        let end_tag = stringify!($end_tag);
        $crate::view::macros::child_to_parent(&mut $stack, Some(end_tag));
        view_internal! { $stack ($($tail)*) }
    );
    // Component closing
    ($stack:ident (< / { $end_component:ty } > $($tail:tt)*)) => (
        $crate::view::macros::child_to_parent_component(
            &mut $stack,
            $crate::view::macros::TypeId::of::<$end_component>()
        );
        view_internal! { $stack ($($tail)*) }
    );
    // end of paring rule
    ($stack:ident ()) => (
        $crate::view::macros::unwrap($stack)
    );
}

#[macro_export]
macro_rules! view {
    ($($tail:tt)*) => ({
        let mut stack = Vec::new();
        view_internal! { stack ($($tail)*) }
    });
}
