use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use fnv::FnvHashMap;

use super::Node;

#[derive(Clone)]
pub struct Nodes(Arc<RwLock<FnvHashMap<String, Node>>>);

impl Nodes {
    #[inline]
    pub fn new() -> Self {
        Nodes(Arc::new(RwLock::new(FnvHashMap::default())))
    }

    #[inline]
    pub fn read(&self) -> RwLockReadGuard<FnvHashMap<String, Node>> {
        self.0.read().expect("failed to acquire nodes read lock")
    }
    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<FnvHashMap<String, Node>> {
        self.0.write().expect("failed to acquire nodes read lock")
    }

    #[inline]
    pub fn insert(&self, id: String, node: Node) {
        self.write().insert(id, node);
    }
    #[inline]
    pub fn remove(&self, id: &str) {
        self.write().remove(id);
    }
    #[inline]
    pub fn get(&self, id: &str) -> Option<Node> {
        self.write().get(id).map(Clone::clone)
    }
}
