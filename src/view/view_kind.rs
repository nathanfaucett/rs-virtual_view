use std::any::TypeId;
use std::fmt;
use std::sync::Arc;

use super::Component;

#[derive(Clone)]
pub enum ViewKind {
    String(String),
    Component(Arc<dyn Component>),
}

unsafe impl Sync for ViewKind {}
unsafe impl Send for ViewKind {}

impl<'a> From<&'a str> for ViewKind {
    #[inline(always)]
    fn from(string: &'a str) -> Self {
        ViewKind::String(string.to_owned())
    }
}

impl From<String> for ViewKind {
    #[inline(always)]
    fn from(string: String) -> Self {
        ViewKind::String(string)
    }
}

impl From<Arc<dyn Component>> for ViewKind {
    #[inline]
    fn from(component: Arc<dyn Component>) -> Self {
        ViewKind::Component(component)
    }
}

impl<T> From<T> for ViewKind
where
    T: Component,
{
    #[inline]
    fn from(component: T) -> Self {
        ViewKind::Component(Arc::new(component))
    }
}

impl PartialEq for ViewKind {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match self {
            &ViewKind::String(ref a) => match other {
                &ViewKind::String(ref b) => a == b,
                &ViewKind::Component(_) => false,
            },
            &ViewKind::Component(ref a) => match other {
                &ViewKind::String(_) => false,
                &ViewKind::Component(ref b) => a.type_id() == b.type_id(),
            },
        }
    }
}
impl PartialEq<str> for ViewKind {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        match self {
            &ViewKind::String(ref a) => a == other,
            &ViewKind::Component(_) => false,
        }
    }
}

impl fmt::Debug for ViewKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ViewKind::String(ref string) => f.write_str(string),
            &ViewKind::Component(ref component) => write!(f, "Component({})", component.name()),
        }
    }
}

impl fmt::Display for ViewKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl ViewKind {
    #[inline]
    pub fn is_string(&self) -> bool {
        match self {
            &ViewKind::String(_) => true,
            &ViewKind::Component(_) => false,
        }
    }
    #[inline]
    pub fn is_component(&self) -> bool {
        match self {
            &ViewKind::String(_) => false,
            &ViewKind::Component(_) => true,
        }
    }
    #[inline]
    pub fn type_id(&self) -> TypeId {
        match self {
            &ViewKind::String(_) => TypeId::of::<String>(),
            &ViewKind::Component(ref component) => (&**component).type_id(),
        }
    }
    #[inline]
    pub fn take_string(self) -> String {
        match self {
            ViewKind::String(string) => string,
            ViewKind::Component(_) => panic!("ViewKind::Component can not be a String"),
        }
    }
}
