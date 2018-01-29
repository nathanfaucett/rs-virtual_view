use super::super::{EventManager, Transaction, View};
use super::{Node, Nodes};

pub struct ViewNode {
    id: String,
    parent_id: String,
    view: View,
}

impl Node for ViewNode {
    #[inline(always)]
    fn id(&self) -> &String {
        &self.id
    }
    #[inline(always)]
    fn parent_id(&self) -> &String {
        &self.parent_id
    }
    #[inline]
    fn mount(
        &mut self,
        nodes: &Nodes,
        transaction: &mut Transaction,
        event_manager: &mut EventManager,
    ) {
    }
}

impl ViewNode {
    #[inline]
    pub fn new(id: String, parent_id: String, view: View) -> Self {
        ViewNode {
            id: id,
            parent_id: parent_id,
            view: view,
        }
    }
}
