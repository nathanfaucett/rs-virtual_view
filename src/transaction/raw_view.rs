use serde_json::{Map, Value};

use super::super::{props_to_json, View};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RawView {
    Text(String),
    Data {
        kind: String,
        key: Option<String>,
        props: Map<String, Value>,
        children: Vec<RawView>,
    },
}

unsafe impl Sync for RawView {}
unsafe impl Send for RawView {}

impl<'a> From<&'a View> for RawView {
    #[inline]
    fn from(view: &'a View) -> Self {
        match view {
            &View::Text(ref text) => RawView::Text(text.clone()),
            &View::Data {
                ref kind,
                ref key,
                ref props,
                ref children,
                ..
            } => RawView::Data {
                kind: kind.to_string(),
                key: key.clone(),
                props: props_to_json(props),
                children: children.iter().map(|child| RawView::from(child)).collect(),
            },
        }
    }
}

impl From<View> for RawView {
    #[inline]
    fn from(view: View) -> Self {
        match view {
            View::Text(text) => RawView::Text(text),
            View::Data {
                kind,
                key,
                props,
                children,
                ..
            } => RawView::Data {
                kind: kind.take_string(),
                key: key,
                props: props_to_json(&props),
                children: children
                    .into_iter()
                    .map(|child| RawView::from(child))
                    .collect(),
            },
        }
    }
}

impl RawView {
    #[inline]
    pub fn kind(&self) -> Option<&String> {
        match self {
            &RawView::Text(_) => None,
            &RawView::Data { ref kind, .. } => Some(kind),
        }
    }
    #[inline]
    pub fn key(&self) -> Option<&String> {
        match self {
            &RawView::Text(_) => None,
            &RawView::Data { ref key, .. } => key.as_ref(),
        }
    }
    #[inline]
    pub fn props(&self) -> Option<&Map<String, Value>> {
        match self {
            &RawView::Text(_) => None,
            &RawView::Data { ref props, .. } => Some(props),
        }
    }
    #[inline]
    pub fn children(&self) -> Option<&Vec<RawView>> {
        match self {
            &RawView::Text(_) => None,
            &RawView::Data { ref children, .. } => Some(children),
        }
    }
}
