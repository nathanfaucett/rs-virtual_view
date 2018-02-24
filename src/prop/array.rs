use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

use serde_json::Value;

use super::{array_to_json, Prop};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Array(Vec<Prop>);

unsafe impl Sync for Array {}
unsafe impl Send for Array {}

impl Array {
    #[inline(always)]
    pub fn new() -> Self {
        Array(Vec::new())
    }
    #[inline(always)]
    pub fn with_capacity(cap: usize) -> Self {
        Array(Vec::with_capacity(cap))
    }

    #[inline]
    pub fn push<T>(&mut self, value: T)
    where
        T: Into<Prop>,
    {
        self.0.push(value.into())
    }

    #[inline]
    pub fn to_json(&self) -> Value {
        Value::Array(array_to_json(self))
    }
}

impl Deref for Array {
    type Target = Vec<Prop>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Array {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Hash for Array {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        for v in self {
            v.hash(state);
        }
    }
}

impl IntoIterator for Array {
    type Item = Prop;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> IntoIterator for &'a Array {
    type Item = &'a Prop;
    type IntoIter = ::std::slice::Iter<'a, Prop>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<'a> IntoIterator for &'a mut Array {
    type Item = &'a mut Prop;
    type IntoIter = ::std::slice::IterMut<'a, Prop>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T> From<Vec<T>> for Array
where
    T: Into<Prop>,
{
    #[inline(always)]
    fn from(vec: Vec<T>) -> Self {
        let mut a = Array::new();
        for v in vec {
            a.push(v.into());
        }
        a
    }
}

impl<T> FromIterator<T> for Array
where
    T: Into<Prop>,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut a = Array::new();
        for v in iter {
            a.push(v.into());
        }
        a
    }
}
