use std::{fmt, mem, ptr};
use std::sync::Arc;
use std::hash::{Hash, Hasher};

use fnv::FnvHashMap;
use serde_json::{self, Map, Value};

use super::{Array, Function, Number, Props};

#[derive(Clone)]
pub enum Prop {
    Null,
    Boolean(bool),
    Number(Number),
    String(String),
    Function(Arc<Function>),
    Array(Array),
    Object(Props),
}

unsafe impl Sync for Prop {}
unsafe impl Send for Prop {}

impl Prop {
    #[inline]
    pub fn null(&self) -> Option<()> {
        match self {
            &Prop::Null => Some(()),
            _ => None,
        }
    }
    #[inline]
    pub fn boolean(&self) -> Option<bool> {
        match self {
            &Prop::Boolean(v) => Some(v),
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
    pub fn function(&self) -> Option<&Arc<Function>> {
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
    pub fn object(&self) -> Option<&Props> {
        match self {
            &Prop::Object(ref v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn array_mut(&mut self) -> Option<&mut Array> {
        match self {
            &mut Prop::Array(ref mut v) => Some(v),
            _ => None,
        }
    }
    #[inline]
    pub fn object_mut(&mut self) -> Option<&mut Props> {
        match self {
            &mut Prop::Object(ref mut v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn take_null(self) -> Result<(), Self> {
        match self {
            Prop::Null => Ok(()),
            _ => Err(self),
        }
    }
    #[inline]
    pub fn take_boolean(self) -> Result<bool, Self> {
        match self {
            Prop::Boolean(v) => Ok(v),
            _ => Err(self),
        }
    }
    #[inline]
    pub fn take_number(self) -> Result<Number, Self> {
        match self {
            Prop::Number(v) => Ok(v),
            _ => Err(self),
        }
    }
    #[inline]
    pub fn take_string(self) -> Result<String, Self> {
        match self {
            Prop::String(v) => Ok(v),
            _ => Err(self),
        }
    }
    #[inline]
    pub fn take_function(self) -> Result<Arc<Function>, Self> {
        match self {
            Prop::Function(v) => Ok(v),
            _ => Err(self),
        }
    }
    #[inline]
    pub fn take_array(self) -> Result<Array, Self> {
        match self {
            Prop::Array(v) => Ok(v),
            _ => Err(self),
        }
    }
    #[inline]
    pub fn take_object(self) -> Result<Props, Self> {
        match self {
            Prop::Object(v) => Ok(v),
            _ => Err(self),
        }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        match self {
            &Prop::Null => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_boolean(&self) -> bool {
        match self {
            &Prop::Boolean(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_number(&self) -> bool {
        match self {
            &Prop::Number(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_string(&self) -> bool {
        match self {
            &Prop::String(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_function(&self) -> bool {
        match self {
            &Prop::Function(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_array(&self) -> bool {
        match self {
            &Prop::Array(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_object(&self) -> bool {
        match self {
            &Prop::Object(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn call(&self, e: &mut Props) -> Option<Prop> {
        match self {
            &Prop::Function(ref f) => Some((&**f)(e)),
            _ => None,
        }
    }

    #[inline]
    pub fn is_truthy(&self) -> bool {
        match self {
            &Prop::Null => false,
            &Prop::Boolean(ref v) => *v,
            &Prop::Number(ref v) => v != &0.0,
            &Prop::String(ref v) => !v.is_empty(),
            &Prop::Function(_) => true,
            &Prop::Array(ref v) => !v.is_empty(),
            &Prop::Object(ref v) => !v.is_empty(),
        }
    }
    #[inline]
    pub fn is_falsey(&self) -> bool {
        !self.is_truthy()
    }

    #[inline]
    pub fn is_true(&self) -> bool {
        match self {
            &Prop::Boolean(ref v) => v == &true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_false(&self) -> bool {
        match self {
            &Prop::Boolean(ref v) => v == &false,
            _ => false,
        }
    }

    #[inline]
    pub fn to_json(&self) -> Value {
        prop_to_json(self)
    }
}

impl<'a> From<&'a Prop> for Prop {
    #[inline]
    fn from(prop: &'a Prop) -> Self {
        prop.clone()
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
        Prop::Boolean(value)
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

impl From<Array> for Prop {
    #[inline]
    fn from(array: Array) -> Self {
        Prop::Array(array)
    }
}

impl<T> From<FnvHashMap<String, T>> for Prop
where
    T: Into<Prop>,
{
    #[inline]
    fn from(value: FnvHashMap<String, T>) -> Self {
        Prop::Object(value.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

impl From<Props> for Prop {
    #[inline]
    fn from(props: Props) -> Self {
        Prop::Object(props)
    }
}

impl<F> From<F> for Prop
where
    F: 'static + Fn(&mut Props) -> Prop,
{
    #[inline]
    fn from(f: F) -> Self {
        Prop::Function(Arc::new(f))
    }
}

impl From<Value> for Prop {
    #[inline]
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Prop::Null,
            Value::Bool(v) => Prop::Boolean(v),
            Value::Number(v) => Prop::Number(v.as_f64().unwrap_or(0.0)),
            Value::String(v) => Prop::String(v),
            Value::Array(a) => Prop::Array(a.into_iter().map(Into::<Prop>::into).collect()),
            Value::Object(m) => Prop::Object(
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
            &Value::Bool(ref v) => Prop::Boolean(*v),
            &Value::Number(ref v) => Prop::Number(v.as_f64().unwrap_or(0.0)),
            &Value::String(ref v) => Prop::String(v.clone()),
            &Value::Array(ref a) => Prop::Array(a.into_iter().map(Into::<Prop>::into).collect()),
            &Value::Object(ref m) => Prop::Object(
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
            &Prop::Boolean(ref v) => write!(f, "{:?}", v),
            &Prop::Number(ref v) => write!(f, "{:?}", v),
            &Prop::String(ref v) => write!(f, "{:?}", v),
            &Prop::Function(_) => write!(f, "Fn(&mut Props) -> Prop"),
            &Prop::Array(ref v) => write!(f, "{:?}", v),
            &Prop::Object(ref v) => write!(f, "{:?}", v),
        }
    }
}

impl fmt::Display for Prop {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Prop::Null => write!(f, "null"),
            &Prop::Boolean(ref v) => write!(f, "{}", v),
            &Prop::Number(ref v) => write!(f, "{}", v),
            &Prop::String(ref v) => write!(f, "{}", v),
            &Prop::Function(_) => write!(f, "Fn(&mut Props) -> Prop"),
            &Prop::Array(ref array) => {
                let il = array.len();

                if il != 0 {
                    let mut out = String::new();
                    let mut i = 1;

                    out.push_str(&array[0].to_string());

                    while i < il {
                        out.push_str(", ");
                        out.push_str(&array[i].to_string());
                        i += 1;
                    }

                    write!(f, "[{}]", out)
                } else {
                    write!(f, "[]")
                }
            }
            &Prop::Object(ref object) => {
                let il = object.len();

                if il != 0 {
                    let array = object.iter().collect::<Vec<_>>();

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
                        i += 1;
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
            &Prop::Boolean(ref a) => match other {
                &Prop::Boolean(ref b) => a == b,
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
            &Prop::Object(ref a) => match other {
                &Prop::Object(ref b) => a == b,
                _ => false,
            },
        }
    }
}

impl PartialEq<str> for Prop {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        match self {
            &Prop::String(ref string) => string == other,
            _ => &self.to_string() == other,
        }
    }
}

macro_rules! impl_partial_eq_number {
    ($($T:ty),*) => (
        $(impl PartialEq<$T> for Prop {
            #[inline]
            fn eq(&self, other: &$T) -> bool {
                match self {
                    &Prop::Number(ref n) => *n == *other as f64,
                    _ => self.to_string() == other.to_string(),
                }
            }
        })*
    );
}

impl_partial_eq_number!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

impl Hash for Prop {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            &Prop::Null => ().hash(state),
            &Prop::Boolean(ref v) => v.hash(state),
            &Prop::Number(ref v) => unsafe { mem::transmute::<f64, u64>(*v) }.hash(state),
            &Prop::String(ref v) => v.hash(state),
            &Prop::Function(ref v) => (&**v as *const _ as *const usize as usize).hash(state),
            &Prop::Array(ref a) => for v in a {
                v.hash(state);
            },
            &Prop::Object(ref m) => for (k, v) in m {
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
        &Prop::Boolean(ref v) => Value::Bool(*v),
        &Prop::Number(ref v) => Value::Number(serde_json::Number::from_f64(*v).unwrap()),
        &Prop::String(ref v) => Value::String(v.clone()),
        &Prop::Function(_) => Value::Null,
        &Prop::Array(ref v) => Value::Array(array_to_json(v)),
        &Prop::Object(ref v) => Value::Object(props_to_json(v)),
    }
}

#[inline]
pub fn array_to_json(array: &Array) -> Vec<Value> {
    let mut out = Vec::new();

    for v in array {
        match v {
            &Prop::Null => out.push(Value::Null),
            &Prop::Boolean(ref v) => out.push(Value::Bool(*v)),
            &Prop::Number(ref v) => {
                out.push(Value::Number(serde_json::Number::from_f64(*v).unwrap()))
            }
            &Prop::String(ref v) => out.push(Value::String(v.clone())),
            &Prop::Function(_) => (),
            &Prop::Array(ref v) => out.push(Value::Array(array_to_json(v))),
            &Prop::Object(ref v) => out.push(Value::Object(props_to_json(v))),
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
            &Prop::Boolean(ref v) => {
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
            &Prop::Function(_) => (),
            &Prop::Array(ref v) => {
                out.insert(k.clone(), Value::Array(array_to_json(v)));
            }
            &Prop::Object(ref v) => {
                out.insert(k.clone(), Value::Object(props_to_json(v)));
            }
        }
    }

    out
}
