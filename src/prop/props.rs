use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

use fnv::FnvHashMap;
use serde_json::{Map, Value};

use super::{props_to_json, Prop};

const PROP_NULL: Prop = Prop::Null;

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
    pub fn has(&self, key: &str) -> bool {
        if let Some(prop) = self.0.get(key) {
            prop != &Prop::Null
        } else {
            false
        }
    }

    #[inline]
    pub fn take(&self, key: &str) -> Option<Prop> {
        self.0.get(key).map(Clone::clone)
    }
    #[inline]
    pub fn get(&self, key: &str) -> &Prop {
        if let Some(prop) = self.0.get(key) {
            prop
        } else {
            &PROP_NULL
        }
    }
    #[inline]
    pub fn get_mut(&mut self, key: &str) -> &mut Prop {
        self.0.entry(key.into()).or_insert(Prop::Null)
    }

    #[inline]
    pub fn update<F>(&mut self, key: &str, f: F) -> &mut Self
    where
        F: Fn(&mut Prop),
    {
        self.0.get_mut(key).map(f);
        self
    }
}

impl<'a> Index<&'a str> for Props {
    type Output = Prop;

    #[inline]
    fn index(&self, key: &'a str) -> &Self::Output {
        self.get(key)
    }
}

impl<'a> IndexMut<&'a str> for Props {
    #[inline]
    fn index_mut(&mut self, key: &'a str) -> &mut Self::Output {
        self.get_mut(key)
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

impl Into<Map<String, Value>> for Props {
    #[inline]
    fn into(self) -> Map<String, Value> {
        props_to_json(&self)
    }
}

impl<'a> Into<Map<String, Value>> for &'a Props {
    #[inline]
    fn into(self) -> Map<String, Value> {
        props_to_json(self)
    }
}
