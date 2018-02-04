use fnv::FnvHashMap;

use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

use super::Prop;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Props(FnvHashMap<String, Prop>);

impl Props {
    #[inline(always)]
    pub fn new() -> Self {
        Props(FnvHashMap::default())
    }

    #[inline]
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: ToString,
        V: Into<Prop>,
    {
        self.0.insert(key.to_string(), value.into());
    }

    #[inline]
    pub fn take(&self, key: &str) -> Option<Prop> {
        self.0.get(key).map(|x| x.clone())
    }
}

impl Deref for Props {
    type Target = FnvHashMap<String, Prop>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Props {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Hash for Props {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        for (k, v) in self {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl IntoIterator for Props {
    type Item = (String, Prop);
    type IntoIter = ::std::collections::hash_map::IntoIter<String, Prop>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> IntoIterator for &'a Props {
    type Item = (&'a String, &'a Prop);
    type IntoIter = ::std::collections::hash_map::Iter<'a, String, Prop>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<'a> IntoIterator for &'a mut Props {
    type Item = (&'a String, &'a mut Prop);
    type IntoIter = ::std::collections::hash_map::IterMut<'a, String, Prop>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<K, V> From<FnvHashMap<K, V>> for Props
where
    K: Eq + Hash + ToString,
    V: Into<Prop>,
{
    #[inline(always)]
    fn from(map: FnvHashMap<K, V>) -> Self {
        let mut m = Props::new();
        for (k, v) in map {
            m.insert(k, v);
        }
        m
    }
}

impl<K, V> FromIterator<(K, V)> for Props
where
    K: Eq + Hash + ToString,
    V: Into<Prop>,
{
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut m = Props::new();
        for (k, v) in iter {
            m.insert(k, v);
        }
        m
    }
}
