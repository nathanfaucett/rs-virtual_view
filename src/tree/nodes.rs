use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use fnv::FnvHashMap;

use super::Node;

#[derive(Clone)]
pub struct Nodes(Arc<RwLock<FnvHashMap<(String, usize), Node>>>);

impl Nodes {
    #[inline]
    pub fn new() -> Self {
        Nodes(Arc::new(RwLock::new(FnvHashMap::default())))
    }

    #[inline]
    pub fn read(&self) -> RwLockReadGuard<FnvHashMap<(String, usize), Node>> {
        self.0.read().expect("failed to acquire nodes read lock")
    }
    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<FnvHashMap<(String, usize), Node>> {
        self.0.write().expect("failed to acquire nodes read lock")
    }

    #[inline]
    pub fn insert(&self, id: String, depth: usize, node: Node) {
        self.write().insert((id, depth), node);
    }

    #[inline]
    pub fn remove_at_depth(&self, id: String, depth: usize) -> Option<Node> {
        self.write().remove(&(id, depth))
    }
    #[inline]
    pub fn remove(&self, id: String) -> Option<Node> {
        self.remove_at_depth(id, 0)
    }

    #[inline]
    pub fn get_at_depth(&self, id: String, depth: usize) -> Option<Node> {
        self.write().get(&(id, depth)).map(Clone::clone)
    }
    #[inline]
    pub fn get(&self, id: String) -> Option<Node> {
        self.get_at_depth(id, 0)
    }
}
