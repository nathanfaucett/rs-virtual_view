use std::sync::Arc;
use std::fmt;

use super::super::prop_to_string_take;
use super::{Children, Component, Props, ViewKind};

pub enum View {
    Text(String),
    Data {
        kind: ViewKind,
        key: Option<String>,
        props: Props,
        children: Children,
    },
}

impl fmt::Debug for View {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &View::Text(ref string) => write!(f, "Text({})", string),
            &View::Data {
                ref kind,
                ref key,
                ref props,
                ref children,
            } => f.debug_struct("Data")
                .field("kind", kind)
                .field("key", key)
                .field("props", props)
                .field("children", children)
                .finish(),
        }
    }
}

impl Clone for View {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            &View::Text(ref string) => View::Text(string.clone()),
            &View::Data {
                ref kind,
                ref key,
                ref props,
                ref children,
                ..
            } => View::Data {
                kind: kind.clone(),
                key: key.clone(),
                props: props.clone(),
                children: children.clone(),
            },
        }
    }
}

impl PartialEq for View {
    #[inline]
    fn eq(&self, other: &View) -> bool {
        match self {
            &View::Text(ref lhs_string) => match other {
                &View::Text(ref rhs_string) => lhs_string == rhs_string,
                &View::Data { .. } => false,
            },
            &View::Data {
                kind: ref lhs_kind,
                key: ref lhs_key,
                props: ref lhs_props,
                children: ref lhs_children,
                ..
            } => match self {
                &View::Text(_) => false,
                &View::Data {
                    kind: ref rhs_kind,
                    key: ref rhs_key,
                    props: ref rhs_props,
                    children: ref rhs_children,
                } => {
                    lhs_kind == rhs_kind && lhs_key == rhs_key && lhs_props == rhs_props
                        && lhs_children == rhs_children
                }
            },
        }
    }
}

impl View {
    #[inline]
    pub fn new(kind: ViewKind, mut props: Props, children: Children) -> Self {
        let key = props.remove("key").map(prop_to_string_take);

        View::Data {
            kind: kind,
            key: key,
            props: props,
            children: children,
        }
    }

    #[inline]
    pub fn new_text(text: &str) -> Self {
        View::Text(text.into())
    }
    #[inline]
    pub fn new_data(kind: &str) -> Self {
        View::Data {
            kind: kind.into(),
            key: None,
            props: Props::default(),
            children: Children::new(),
        }
    }
    #[inline]
    pub fn new_component<T>(component: T) -> Self
    where
        T: Component,
    {
        View::Data {
            kind: component.into(),
            key: None,
            props: Props::default(),
            children: Children::new(),
        }
    }

    #[inline]
    pub fn kind(&self) -> Option<&ViewKind> {
        match self {
            &View::Text(_) => None,
            &View::Data { ref kind, .. } => Some(kind),
        }
    }
    #[inline]
    pub fn tag(&self) -> Option<&String> {
        match self.kind() {
            Some(&ViewKind::String(ref string)) => Some(string),
            _ => None,
        }
    }
    #[inline]
    pub fn component(&self) -> Option<&Arc<Component>> {
        match self.kind() {
            Some(&ViewKind::Component(ref component)) => Some(component),
            _ => None,
        }
    }

    #[inline]
    pub fn key(&self) -> Option<&String> {
        match self {
            &View::Text(_) => None,
            &View::Data { ref key, .. } => key.as_ref(),
        }
    }
    #[inline]
    pub fn set_key(&mut self, new_key: String) {
        match self {
            &mut View::Text(_) => (),
            &mut View::Data { ref mut key, .. } => *key = Some(new_key),
        }
    }

    #[inline]
    pub fn props(&self) -> Option<&Props> {
        match self {
            &View::Text(_) => None,
            &View::Data { ref props, .. } => Some(props),
        }
    }
    #[inline]
    pub fn props_mut(&mut self) -> Option<&mut Props> {
        match self {
            &mut View::Text(_) => None,
            &mut View::Data { ref mut props, .. } => Some(props),
        }
    }

    #[inline]
    pub fn children(&self) -> Option<&Children> {
        match self {
            &View::Text(_) => None,
            &View::Data { ref children, .. } => Some(children),
        }
    }
    #[inline]
    pub fn children_mut(&mut self) -> Option<&mut Children> {
        match self {
            &mut View::Text(_) => None,
            &mut View::Data {
                ref mut children, ..
            } => Some(children),
        }
    }
}

impl<'a> From<&'a View> for View {
    #[inline]
    fn from(view: &'a View) -> Self {
        view.clone()
    }
}

impl<T> From<T> for View
where
    T: ToString,
{
    #[inline]
    fn from(value: T) -> Self {
        View::Text(value.to_string())
    }
}
