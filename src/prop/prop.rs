use std::{fmt, mem, ptr};
use std::sync::Arc;
use std::hash::{Hash, Hasher};

use fnv::FnvHashMap;
use serde_json::{self, Map, Value};

use super::super::Event;
use super::{Array, Props};

#[derive(Clone)]
pub enum Prop {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Function(Function),
    Array(Array),
    Map(Props),
}

pub type Number = f64;
pub type Function = Arc<Fn(&mut Event)>;

impl Prop {
    #[inline]
    pub fn null(&self) -> Option<()> {
        match self {
            &Prop::Null => Some(()),
            _ => None,
        }
    }
    #[inline]
    pub fn bool(&self) -> Option<bool> {
        match self {
            &Prop::Bool(v) => Some(v),
            _ => None,
        }
    }
    #[inline]
    pub fn number(&self) -> Option<Number> {
        match self {
            &Prop::Number(v) => Some(v),
            _ => None,
        }
    }
    #[inline]
    pub fn string(&self) -> Option<&String> {
        match self {
            &Prop::String(ref v) => Some(v),
            _ => None,
        }
    }
    #[inline]
    pub fn function(&self) -> Option<&Function> {
        match self {
            &Prop::Function(ref v) => Some(v),
            _ => None,
        }
    }
    #[inline]
    pub fn array(&self) -> Option<&Array> {
        match self {
            &Prop::Array(ref v) => Some(v),
            _ => None,
        }
    }
    #[inline]
    pub fn map(&self) -> Option<&Props> {
        match self {
            &Prop::Map(ref v) => Some(v),
            _ => None,
        }
    }
}

impl From<()> for Prop {
    #[inline]
    fn from(_: ()) -> Self {
        Prop::Null
    }
}

impl From<bool> for Prop {
    #[inline]
    fn from(value: bool) -> Self {
        Prop::Bool(value)
    }
}

impl<'a> From<&'a str> for Prop {
    #[inline]
    fn from(value: &'a str) -> Self {
        Prop::String(value.to_owned())
    }
}
impl From<String> for Prop {
    #[inline]
    fn from(value: String) -> Self {
        Prop::String(value)
    }
}

macro_rules! impl_from_number {
    ($($T:ty),*) => (
        $(impl From<$T> for Prop {
            #[inline]
            fn from(value: $T) -> Self {
                Prop::Number(value as f64)
            }
        })*
    );
}

impl_from_number!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

impl<T> From<Vec<T>> for Prop
where
    T: Into<Prop>,
{
    #[inline]
    fn from(value: Vec<T>) -> Self {
        Prop::Array(value.into_iter().map(Into::into).collect())
    }
}

impl<T> From<FnvHashMap<String, T>> for Prop
where
    T: Into<Prop>,
{
    #[inline]
    fn from(value: FnvHashMap<String, T>) -> Self {
        Prop::Map(value.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

impl<F> From<F> for Prop
where
    F: 'static + Fn(&mut Event),
{
    #[inline]
    fn from(value: F) -> Self {
        Prop::Function(Arc::new(value))
    }
}

impl From<Value> for Prop {
    #[inline]
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Prop::Null,
            Value::Bool(v) => Prop::Bool(v),
            Value::Number(v) => Prop::Number(v.as_f64().unwrap_or(0.0)),
            Value::String(v) => Prop::String(v),
            Value::Array(a) => Prop::Array(a.into_iter().map(Into::<Prop>::into).collect()),
            Value::Object(m) => Prop::Map(
                m.into_iter()
                    .map(|(k, v)| (k, Into::<Prop>::into(v)))
                    .collect(),
            ),
        }
    }
}

impl<'a> From<&'a Value> for Prop {
    #[inline]
    fn from(value: &'a Value) -> Self {
        match value {
            &Value::Null => Prop::Null,
            &Value::Bool(ref v) => Prop::Bool(*v),
            &Value::Number(ref v) => Prop::Number(v.as_f64().unwrap_or(0.0)),
            &Value::String(ref v) => Prop::String(v.clone()),
            &Value::Array(ref a) => Prop::Array(a.into_iter().map(Into::<Prop>::into).collect()),
            &Value::Object(ref m) => Prop::Map(
                m.into_iter()
                    .map(|(k, v)| (k.clone(), Into::<Prop>::into(v)))
                    .collect(),
            ),
        }
    }
}

impl fmt::Debug for Prop {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Prop::Null => write!(f, "null"),
            &Prop::Bool(ref v) => write!(f, "{:?}", v),
            &Prop::Number(ref v) => write!(f, "{:?}", v),
            &Prop::String(ref v) => write!(f, "{:?}", v),
            &Prop::Function(_) => write!(f, "Fn(&mut Event)"),
            &Prop::Array(ref v) => write!(f, "{:?}", v),
            &Prop::Map(ref v) => write!(f, "{:?}", v),
        }
    }
}

impl fmt::Display for Prop {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Prop::Null => write!(f, "null"),
            &Prop::Bool(ref v) => write!(f, "{}", v),
            &Prop::Number(ref v) => write!(f, "{}", v),
            &Prop::String(ref v) => write!(f, "{}", v),
            &Prop::Function(_) => write!(f, "Fn(&mut Event)"),
            &Prop::Array(ref array) => {
                let il = array.len();

                if il != 0 {
                    let mut out = String::new();
                    let mut i = 1;

                    out.push_str(&array[0].to_string());

                    while i < il {
                        out.push_str(", ");
                        out.push_str(&array[i].to_string());
                    }

                    write!(f, "[{}]", out)
                } else {
                    write!(f, "[]")
                }
            }
            &Prop::Map(ref map) => {
                let il = map.len();

                if il != 0 {
                    let array = map.iter().collect::<Vec<_>>();

                    let mut out = String::new();
                    let mut i = 1;

                    let &(ref k, ref v) = &array[0];

                    out.push_str(&k.to_string());
                    out.push_str(" => ");
                    out.push_str(&v.to_string());

                    while i < il {
                        let &(ref k, ref v) = &array[i];
                        out.push_str(", ");
                        out.push_str(&k.to_string());
                        out.push_str(" => ");
                        out.push_str(&v.to_string());
                    }

                    write!(f, "[{}]", out)
                } else {
                    write!(f, "[]")
                }
            }
        }
    }
}

impl Eq for Prop {}

impl PartialEq for Prop {
    #[inline]
    fn eq(&self, other: &Prop) -> bool {
        match self {
            &Prop::Null => match other {
                &Prop::Null => true,
                _ => false,
            },
            &Prop::Bool(ref a) => match other {
                &Prop::Bool(ref b) => a == b,
                _ => false,
            },
            &Prop::Number(ref a) => match other {
                &Prop::Number(ref b) => a == b,
                _ => false,
            },
            &Prop::String(ref a) => match other {
                &Prop::String(ref b) => a == b,
                _ => false,
            },
            &Prop::Function(ref a) => match other {
                &Prop::Function(ref b) => ptr::eq(&**a, &**b),
                _ => false,
            },
            &Prop::Array(ref a) => match other {
                &Prop::Array(ref b) => a == b,
                _ => false,
            },
            &Prop::Map(ref a) => match other {
                &Prop::Map(ref b) => a == b,
                _ => false,
            },
        }
    }
}

impl Hash for Prop {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            &Prop::Null => ().hash(state),
            &Prop::Bool(ref v) => v.hash(state),
            &Prop::Number(ref v) => unsafe { mem::transmute::<f64, u64>(*v) }.hash(state),
            &Prop::String(ref v) => v.hash(state),
            &Prop::Function(ref v) => (&**v as *const _ as *const usize as usize).hash(state),
            &Prop::Array(ref a) => for v in a {
                v.hash(state);
            },
            &Prop::Map(ref m) => for (k, v) in m {
                k.hash(state);
                v.hash(state);
            },
        }
    }
}

#[inline]
pub fn prop_to_json(prop: &Prop) -> Value {
    match prop {
        &Prop::Null => Value::Null,
        &Prop::Bool(ref v) => Value::Bool(*v),
        &Prop::Number(ref v) => Value::Number(serde_json::Number::from_f64(*v).unwrap()),
        &Prop::String(ref v) => Value::String(v.clone()),
        &Prop::Function(ref v) => Value::Null,
        &Prop::Array(ref v) => Value::Array(array_to_json(v)),
        &Prop::Map(ref v) => Value::Object(props_to_json(v)),
    }
}

#[inline]
pub fn array_to_json(array: &Array) -> Vec<Value> {
    let mut out = Vec::new();

    for v in array {
        match v {
            &Prop::Null => out.push(Value::Null),
            &Prop::Bool(ref v) => out.push(Value::Bool(*v)),
            &Prop::Number(ref v) => {
                out.push(Value::Number(serde_json::Number::from_f64(*v).unwrap()))
            }
            &Prop::String(ref v) => out.push(Value::String(v.clone())),
            &Prop::Function(ref v) => (),
            &Prop::Array(ref v) => out.push(Value::Array(array_to_json(v))),
            &Prop::Map(ref v) => out.push(Value::Object(props_to_json(v))),
        }
    }

    out
}

#[inline]
pub fn props_to_json(props: &Props) -> Map<String, Value> {
    let mut out = Map::new();

    for (k, v) in props {
        match v {
            &Prop::Null => {
                out.insert(k.clone(), Value::Null);
            }
            &Prop::Bool(ref v) => {
                out.insert(k.clone(), Value::Bool(*v));
            }
            &Prop::Number(ref v) => {
                out.insert(
                    k.clone(),
                    Value::Number(serde_json::Number::from_f64(*v).unwrap()),
                );
            }
            &Prop::String(ref v) => {
                out.insert(k.clone(), Value::String(v.clone()));
            }
            &Prop::Function(ref v) => (),
            &Prop::Array(ref v) => {
                out.insert(k.clone(), Value::Array(array_to_json(v)));
            }
            &Prop::Map(ref v) => {
                out.insert(k.clone(), Value::Object(props_to_json(v)));
            }
        }
    }

    out
}
