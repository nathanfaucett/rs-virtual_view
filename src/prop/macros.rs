// https://github.com/serde-rs/json/blob/master/src/macros.rs

#[macro_export]
#[doc(hidden)]
macro_rules! prop_internal {

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => (
        $crate::Array::from(vec![$($elems,)*])
    );

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => (
        $crate::Array::from(vec![$($elems),*])
    );

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => (
        prop_internal!(@array [$($elems,)* prop_internal!(null)] $($rest)*)
    );

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => (
        prop_internal!(@array [$($elems,)* prop_internal!(true)] $($rest)*)
    );

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => (
        prop_internal!(@array [$($elems,)* prop_internal!(false)] $($rest)*)
    );

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => (
        prop_internal!(@array [$($elems,)* prop_internal!([$($array)*])] $($rest)*)
    );

    // Next element is a object.
    (@array [$($elems:expr,)*] {$($object:tt)*} $($rest:tt)*) => (
        prop_internal!(@array [$($elems,)* prop_internal!({$($object)*})] $($rest)*)
    );

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => (
        prop_internal!(@array [$($elems,)* prop_internal!($next),] $($rest)*)
    );

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => (
        prop_internal!(@array [$($elems,)* prop_internal!($last)])
    );

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => (
        prop_internal!(@array [$($elems,)*] $($rest)*)
    );

    // Done.
    (@object $object:ident () () ()) => ();

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => (
        $object.insert(($($key)+), $value);
        prop_internal!(@object $object () ($($rest)*) ($($rest)*));
    );

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => (
        $object.insert(($($key)+), $value);
    );

    // Extend next value is an expression followed by comma.
    (@object $object:ident (...) (... $value:expr , $($rest:tt)*) $copy:tt) => (
        $object.extend($value);
        prop_internal!(@object $object () ($($rest)*) ($($rest)*));
    );

    // Extend last value is an expression with no trailing comma.
    (@object $object:ident (...) (... $value:expr) $copy:tt) => (
        $object.extend($value);
    );

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => (
        prop_internal!(@object $object [$($key)+] (prop_internal!(null)) $($rest)*);
    );

    // Next value is `true`.
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => (
        prop_internal!(@object $object [$($key)+] (prop_internal!(true)) $($rest)*);
    );

    // Next value is `false`.
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => (
        prop_internal!(@object $object [$($key)+] (prop_internal!(false)) $($rest)*);
    );

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => (
        prop_internal!(@object $object [$($key)+] (prop_internal!([$($array)*])) $($rest)*);
    );

    // Next value is a object.
    (@object $object:ident ($($key:tt)+) (: {$($obj:tt)*} $($rest:tt)*) $copy:tt) => (
        prop_internal!(@object $object [$($key)+] (prop_internal!({$($obj)*})) $($rest)*);
    );

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => (
        prop_internal!(@object $object [$($key)+] (prop_internal!($value)) , $($rest)*);
    );

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => (
        prop_internal!(@object $object [$($key)+] (prop_internal!($value)));
    );

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => (
        // "unexpected end of macro invocation"
        prop_internal!();
    );

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => (
        // "unexpected end of macro invocation"
        prop_internal!();
    );

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => (
        // Takes no arguments so "no rules expected the token `:`".
        unimplemented!($colon);
    );

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => (
        // Takes no arguments so "no rules expected the token `,`".
        unimplemented!($comma);
    );

    // Key is extend symbol so merge next expr into object
    (@object $object:ident () (... $($rest:tt)*) $copy:tt) => (
        prop_internal!(@object $object (...) (... $($rest)*) (... $($rest)*));
    );

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => (
        prop_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    );

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => (
        prop_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    );

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: prop_internal!($($props)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => (
        $crate::Prop::Null
    );

    (true) => (
        $crate::Prop::Boolean(true)
    );

    (false) => (
        $crate::Prop::Boolean(false)
    );

    ([]) => (
        $crate::Prop::Array($crate::Array::new())
    );

    ([ $($tt:tt)+ ]) => (
        $crate::Prop::Array(prop_internal!(@array [] $($tt)+))
    );


    (props {}) => (
        $crate::Props::new()
    );
    (props { $($tt:tt)+ }) => ({
        let mut object = $crate::Props::new();
        prop_internal!(@object object () ($($tt)+) ($($tt)+));
        object
    });

    ({}) => (
        $crate::Prop::Object(prop_internal!(props {}))
    );
    ({ $($tt:tt)+ }) => (
        $crate::Prop::Object(prop_internal!(props { $($tt)+ }))
    );

    (| $($tt:tt)+ | $body:expr) => ({
        let f = | $($tt)+ | $body;
        Into::<$crate::Prop>::into(f)
    });
    (move | $($tt:tt)+ | $body:expr) => ({
        let f = move | $($tt)+ | $body;
        Into::<$crate::Prop>::into(f)
    });

    ($other:expr) => (
        Into::<$crate::Prop>::into($other)
    );
}

#[macro_export]
macro_rules! prop {
    ($($tt:tt)+) => (
        prop_internal!($($tt)+)
    );
}

#[macro_export]
macro_rules! props {
    ($($tt:tt)*) => (
        prop_internal!(props { $($tt)* })
    );
}

#[macro_export]
macro_rules! array {
    ($($tt:tt)*) => (
        prop_internal!(@array [ $($tt)* ])
    );
}
