use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};

use fnv::FnvHashMap;

use super::Node;

#[derive(Clone)]
pub struct Nodes(Rc<RefCell<FnvHashMap<(String, usize), Node>>>);

impl Nodes {
    #[inline]
    pub fn new() -> Self {
        Nodes(Rc::new(RefCell::new(FnvHashMap::default())))
    }

    #[inline]
    pub fn as_ref(&self) -> Ref<FnvHashMap<(String, usize), Node>> {
        self.0.borrow()
    }
    #[inline]
    pub fn as_mut(&self) -> RefMut<FnvHashMap<(String, usize), Node>> {
        self.0.borrow_mut()
    }

    #[inline]
    pub fn insert_at_depth(&self, id: String, depth: usize, node: Node) {
        self.as_mut().insert((id, depth), node);
    }

    #[inline]
    pub fn remove_at_depth(&self, id: String, depth: usize) -> Option<Node> {
        let mut nodes_mut = self.as_mut();
        let top_node = nodes_mut.remove(&(id.clone(), depth));

        let mut current_depth = depth + 1;
        while let Some(_) = nodes_mut.remove(&(id.clone(), current_depth)) {
            current_depth += 1;
        }

        top_node
    }

    #[inline]
    pub fn get_at_depth(&self, id: String, depth: usize) -> Option<Node> {
        self.as_ref().get(&(id, depth)).map(Clone::clone)
    }
    #[inline]
    pub fn get(&self, id: String) -> Option<Node> {
        self.get_at_depth(id, 0)
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }
}
