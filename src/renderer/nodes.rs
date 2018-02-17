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
    pub fn insert_at_depth(&self, id: String, depth: usize, node: Node) {
        self.write().insert((id, depth), node);
    }

    #[inline]
    pub fn remove_at_depth(&self, id: String, depth: usize) -> Option<Node> {
        let mut write = self.write();
        let top_node = write.remove(&(id.clone(), depth));

        let mut current_depth = depth + 1;
        while let Some(_) = write.remove(&(id.clone(), current_depth)) {
            current_depth += 1;
        }

        top_node
    }

    #[inline]
    pub fn get_at_depth(&self, id: String, depth: usize) -> Option<Node> {
        self.read().get(&(id, depth)).map(Clone::clone)
    }
    #[inline]
    pub fn get(&self, id: String) -> Option<Node> {
        self.get_at_depth(id, 0)
    }
}
