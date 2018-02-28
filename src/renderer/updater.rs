use std::fmt;
use std::sync::Arc;
use std::hash::{Hash, Hasher};

use serde_json::Value;

use super::super::Props;
use super::Renderer;

pub struct UpdaterInner {
    id: String,
    depth: usize,
    renderer: Renderer,
}

#[derive(Clone)]
pub struct Updater(Arc<UpdaterInner>);

unsafe impl Send for Updater {}
unsafe impl Sync for Updater {}

impl Updater {
    #[inline]
    pub fn new(id: String, depth: usize, renderer: Renderer) -> Self {
        Updater(Arc::new(UpdaterInner {
            id: id,
            depth: depth,
            renderer: renderer,
        }))
    }

    #[inline]
    fn set_json_id<V>(&self, json: V) -> Value
    where
        V: Into<Value>,
    {
        let mut props = json.into();

        match &mut props {
            &mut Value::Object(ref mut object) => {
                object.insert("component_id".into(), self.0.id.clone().into());
            }
            _ => (),
        }

        props
    }

    #[inline]
    pub fn send<N, V, F>(&self, name: N, json: V, f: F)
    where
        N: Into<String>,
        V: Into<Value>,
        F: 'static + Fn(Value),
    {
        self.0.renderer.send(name, self.set_json_id(json), f)
    }

    #[inline]
    pub fn send_no_callback<N, V>(&self, name: N, json: V)
    where
        N: Into<String>,
        V: Into<Value>,
    {
        self.0
            .renderer
            .send_no_callback(name, self.set_json_id(json))
    }

    #[inline]
    pub fn set_state<F>(&self, f: F)
    where
        F: 'static + Send + Fn(&Props) -> Props,
    {
        self.0.renderer.update(self.0.id.clone(), self.0.depth, f)
    }

    #[inline]
    pub fn force_update(&self) {
        self.set_state(Clone::clone);
    }
}

impl PartialEq for Updater {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id && self.0.depth == other.0.depth
    }
}

impl Hash for Updater {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.id.hash(state);
        self.0.depth.hash(state);
    }
}

impl fmt::Debug for Updater {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Updater({}, {})", self.0.id, self.0.depth)
    }
}

impl fmt::Display for Updater {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
