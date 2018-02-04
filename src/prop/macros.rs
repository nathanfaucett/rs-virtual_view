#[macro_export]
macro_rules! props {
    // Hide distracting implementation details from the generated rustdoc.
    ($($props:tt)+) => {
        props_internal!($($props)+)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! props_internal {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: props_internal!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        $crate::Array::from(vec![$($elems,)*])
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        $crate::Array::from(vec![$($elems),*])
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        props_internal!(@array [$($elems,)* props_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        props_internal!(@array [$($elems,)* props_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        props_internal!(@array [$($elems,)* props_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        props_internal!(@array [$($elems,)* props_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        props_internal!(@array [$($elems,)* props_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        props_internal!(@array [$($elems,)* props_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        props_internal!(@array [$($elems,)* props_internal!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        props_internal!(@array [$($elems,)*] $($rest)*)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: props_internal!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@object $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        $object.insert(($($key)+), $value);
        props_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        $object.insert(($($key)+), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        props_internal!(@object $object [$($key)+] (props_internal!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        props_internal!(@object $object [$($key)+] (props_internal!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        props_internal!(@object $object [$($key)+] (props_internal!(false)) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        props_internal!(@object $object [$($key)+] (props_internal!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        props_internal!(@object $object [$($key)+] (props_internal!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        props_internal!(@object $object [$($key)+] (props_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        props_internal!(@object $object [$($key)+] (props_internal!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        props_internal!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        props_internal!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        unimplemented!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        unimplemented!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        props_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        props_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: props_internal!($($props)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        $crate::Prop::Null
    };

    (true) => {
        $crate::Prop::Boolean(true)
    };

    (false) => {
        $crate::Prop::Boolean(false)
    };

    ([]) => {
        $crate::Prop::Array($crate::Array::new())
    };

    ([ $($tt:tt)+ ]) => {
        $crate::Prop::Array(props_internal!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::Prop::Map($crate::Props::new())
    };

    ({ $($tt:tt)+ }) => {
        $crate::Prop::Map({
            let mut object = $crate::Props::new();
            props_internal!(@object object () ($($tt)+) ($($tt)+));
            object
        })
    };

    (| $($tt:tt)+ | $body:expr) => {
        let f = | $($tt)+ | $body;
        Into::<Prop>::into(f)
    };
    (move | $($tt:tt)+ | $body:expr) => {
        let f = move | $($tt)+ | $body;
        Into::<Prop>::into(f)
    };

    ($other:expr) => {
        Into::<Prop>::into($other)
    };
}
