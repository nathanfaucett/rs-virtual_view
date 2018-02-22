use std::{fmt, mem, raw};
use std::intrinsics::type_name;

pub struct AnyFn {
    trait_object: raw::TraitObject,
    name: (&'static str, &'static str),
}

impl fmt::Debug for AnyFn {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fn{} -> {}", self.name.0, self.name.1)
    }
}

impl AnyFn {
    #[inline]
    pub fn new<F, A, R>(f: F) -> Self
    where
        F: 'static + Fn<A, Output = R>,
        A: 'static,
        R: 'static,
    {
        let object: Box<Fn<A, Output = R>> = Box::new(f);
        let trait_object: raw::TraitObject = unsafe { mem::transmute(object) };
        let name = unsafe { (type_name::<A>(), type_name::<R>()) };

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
        self.name == unsafe { (type_name::<A>(), type_name::<R>()) }
    }

    /// # Examples
    /// ```
    /// use virtual_view::AnyFn;
    ///
    /// let add = AnyFn::new(|a: isize, b: isize| -> isize { a + b });
    /// let result: isize = add.call((2_isize, 2_isize)).unwrap();
    /// assert_eq!(result, 4);
    /// ```
    #[inline]
    pub fn call<A, R>(&self, args: A) -> Result<R, String> {
        if self.is::<A, R>() {
            Ok(unsafe { self.call_unchecked(args) })
        } else {
            Err(format!(
                "Invalid Args or Return passed to AnyFn, is {:?} passed {}",
                self,
                unsafe { format!("Fn{} -> {}", type_name::<A>(), type_name::<R>()) }
            ))
        }
    }

    #[inline]
    pub unsafe fn call_unchecked<A, R>(&self, args: A) -> R {
        let f: Box<Fn<A, Output = R>> = mem::transmute(self.trait_object);
        Fn::call(&*f, args)
    }
}
