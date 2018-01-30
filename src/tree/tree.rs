use std::sync::atomic::{AtomicUsize, Ordering};

use super::super::{view_id, EventManager, Transaction, View};
use super::{ComponentNode, Node, Nodes, ViewNode};

static ROOT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Tree {
    root_index: usize,
    root_id: String,
    pub nodes: Nodes,
}

impl Tree {
    #[inline]
    pub fn new() -> Self {
        let mut root_id = String::new();
        let root_index = ROOT_ID.fetch_add(1, Ordering::SeqCst);

        root_id.push('.');
        root_id.push_str(&root_index.to_string());

        Tree {
            root_index: root_index,
            root_id: root_id,
            nodes: Nodes::new(),
        }
    }

    #[inline]
    pub fn render(&self, view: View, event_manager: &mut EventManager) -> Transaction {
        let mut transaction = Transaction::new();

        Self::mount_view(
            &self.nodes,
            self.root_id.clone(),
            view,
            &mut transaction,
            event_manager,
        );

        transaction
    }

    pub(super) fn mount_view(
        nodes: &Nodes,
        id: String,
        view: View,
        transaction: &mut Transaction,
        event_manager: &mut EventManager,
    ) -> View {
        let mut node: Box<Node> = if let Some(component) = view.component().map(|c| c.clone()) {
            Box::new(ComponentNode::new(id.clone(), "".into(), view, component))
        } else {
            Box::new(ViewNode::new(id.clone(), "".into(), view))
        };

        let rendered_view = node.mount(nodes, transaction, event_manager);
        nodes.lock().insert(id, node);

        rendered_view
    }
}
