use std::sync::Arc;

use super::super::Props;
use super::{Children, Component, ViewKind};

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Text(String),
    Data {
        kind: ViewKind,
        key: Option<String>,
        props: Props,
        children: Children,
    },
}

unsafe impl Sync for View {}
unsafe impl Send for View {}

impl View {
    #[inline]
    pub fn new(kind: ViewKind, mut props: Props, children: Children) -> Self {
        let key = props.remove("key").map(|p| match p.take_string() {
            Ok(string) => string,
            Err(p) => p.to_string(),
        });

        View::Data {
            kind: kind,
            key: key,
            props: props,
            children: children,
        }
    }

    #[inline]
    pub fn new_empty() -> Self {
        View::Text(String::new())
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
            props: Props::new(),
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
            props: Props::new(),
            children: Children::new(),
        }
    }

    #[inline]
    pub fn is_text(&self) -> bool {
        match self {
            &View::Text(_) => true,
            &View::Data { .. } => false,
        }
    }
    #[inline]
    pub fn is_data(&self) -> bool {
        match self {
            &View::Text(_) => false,
            &View::Data { .. } => true,
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
    #[inline]
    pub fn clone_no_children(&self) -> Self {
        match self {
            &View::Text(ref string) => View::Text(string.clone()),
            &View::Data {
                ref kind,
                ref key,
                ref props,
                ..
            } => View::Data {
                kind: kind.clone(),
                key: key.clone(),
                props: props.clone(),
                children: Children::new(),
            },
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
