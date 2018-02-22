use std::{mem, raw};
use std::intrinsics::type_name;

pub struct AnyFn {
    trait_object: raw::TraitObject,
    name: &'static str,
}

impl AnyFn {
    #[inline]
    pub fn new<F, A, R>(f: F) -> Self
    where
        F: Fn<A, Output = R>,
    {
        let object: Box<Fn<A, Output = R>> = Box::new(f);
        let trait_object: raw::TraitObject = unsafe { mem::transmute(object) };
        let name = unsafe { type_name::<(A, R)>() };

        AnyFn {
            trait_object: trait_object,
            name: name,
        }
    }

    /// # Examples
    /// ```
    /// use virtual_view::AnyFn;
    ///
    /// let hello = AnyFn::new(|name: String| -> String { format!("Hello, {}!", name) });
    /// assert!(!hello.is::<((),), ()>());
    /// assert!(hello.is::<(String,), String>());
    /// ```
    #[inline]
    pub fn is<A, R>(&self) -> bool {
        self.name == unsafe { type_name::<(A, R)>() }
    }

    /// # Examples
    /// ```
    /// use virtual_view::AnyFn;
    ///
    /// let add = AnyFn::new(|a: isize, b: isize| -> isize { a + b });
    /// let result: Option<isize> = add.call((2_isize, 2_isize));
    /// assert_eq!(result.unwrap(), 4);
    /// ```
    #[inline]
    pub fn call<A, R>(&self, args: A) -> Option<R> {
        if self.is::<A, R>() {
            Some(unsafe { self.call_unchecked(args) })
        } else {
            None
        }
    }

    #[inline]
    pub unsafe fn call_unchecked<A, R>(&self, args: A) -> R {
        let f: Box<Fn<A, Output = R>> = mem::transmute(self.trait_object);
        Fn::call(&*f, args)
    }
}
