use std::sync::{Arc, Mutex, MutexGuard};

use fnv::FnvHashMap;

use super::Node;

#[derive(Clone)]
pub struct Nodes(Arc<Mutex<FnvHashMap<String, Box<Node>>>>);

impl Nodes {
    #[inline]
    pub fn new() -> Self {
        Nodes(Arc::new(Mutex::new(FnvHashMap::default())))
    }

    #[inline]
    pub fn lock(&self) -> MutexGuard<FnvHashMap<String, Box<Node>>> {
        self.0.lock().unwrap()
    }

    #[inline]
    pub fn insert(&self, id: String, node: Box<Node>) {
        self.lock().insert(id, node);
    }
    #[inline]
    pub fn remove(&self, id: &str) {
        self.lock().remove(id);
    }
}
