use std::sync::atomic::{AtomicUsize, Ordering};

use fnv::FnvHashSet;

use super::super::{view_id, EventManager, Props, Transaction, View};
use super::{ComponentNode, Node, Nodes, ViewNode};

static ROOT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Tree {
    root_index: usize,
    root_id: String,
    event_manager: EventManager,
    nodes: Nodes,
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
            event_manager: EventManager::new(),
            nodes: Nodes::new(),
        }
    }

    #[inline(always)]
    pub fn root_index(&self) -> usize {
        self.root_index
    }
    #[inline(always)]
    pub fn root_id(&self) -> &String {
        &self.root_id
    }
    #[inline(always)]
    pub fn event_manager(&self) -> &EventManager {
        &self.event_manager
    }

    #[inline]
    pub fn mount(&self, view: View) -> Transaction {
        let mut transaction = Transaction::new();

        let rendered_view = Self::mount_view(
            &self.nodes,
            &self.event_manager,
            self.root_id.clone(),
            view,
            &mut transaction,
        );
        transaction.mount(&self.root_id, rendered_view.into());

        transaction
    }

    #[inline]
    pub fn unmount(&self) -> Transaction {
        let mut transaction = Transaction::new();

        if let Some(rendered_view) = Self::unmount_view(
            &self.nodes,
            &self.event_manager,
            &self.root_id,
            &mut transaction,
        ) {
            transaction.unmount(&self.root_id, rendered_view.into());
        }

        transaction
    }

    #[inline]
    pub(super) fn mount_props_events(
        event_manager: &EventManager,
        id: &str,
        props: &Props,
        transaction: &mut Transaction,
    ) {
        for (k, v) in props {
            if k.starts_with("on") {
                if let Some(f) = v.function() {
                    transaction.add_event(id, k);
                    event_manager.add(id, k, f.clone());
                }
            }
        }
    }

    #[inline]
    pub(super) fn unmount_props_events(
        event_manager: &EventManager,
        id: &str,
        props: &Props,
        transaction: &mut Transaction,
    ) {
        for (k, v) in props {
            if k.starts_with("on") {
                if let Some(_) = v.function() {
                    transaction.remove_event(id, k);
                    event_manager.remove(id, k);
                }
            }
        }
    }

    #[inline]
    pub(super) fn mount_view(
        nodes: &Nodes,
        event_manager: &EventManager,
        id: String,
        view: View,
        transaction: &mut Transaction,
    ) -> View {
        let mut node: Box<Node> = if let Some(component) = view.component().map(Clone::clone) {
            Box::new(ComponentNode::new(id.clone(), "".into(), view, component))
        } else {
            Box::new(ViewNode::new(id.clone(), "".into(), view))
        };

        let rendered_view = node.mount(nodes, event_manager, transaction);
        nodes.insert(id, node);

        rendered_view
    }

    #[inline]
    fn unmount_view_internal(
        event_manager: &EventManager,
        remove_ids: &mut FnvHashSet<String>,
        parent_id: &str,
        index: usize,
        child: &View,
        transaction: &mut Transaction,
    ) {
        match child {
            &View::Data {
                ref key,
                ref props,
                ref children,
                ..
            } => {
                let child_id = view_id(parent_id, key.as_ref(), index);

                Self::unmount_props_events(event_manager, &child_id, props, transaction);

                for (index, child) in children.iter().enumerate() {
                    Self::unmount_view_internal(
                        event_manager,
                        remove_ids,
                        &child_id,
                        index,
                        child,
                        transaction,
                    );
                }

                remove_ids.insert(child_id);
            }
            _ => (),
        }
    }

    #[inline]
    pub(super) fn unmount_view(
        nodes: &Nodes,
        event_manager: &EventManager,
        id: &String,
        transaction: &mut Transaction,
    ) -> Option<View> {
        let mut nodes_lock = nodes.lock();

        let mut remove_ids = FnvHashSet::default();

        let view = if let Some(node) = nodes_lock.get(id) {
            remove_ids.insert(id.clone());

            if let Some(view) = node.last_rendered_view() {
                if let Some(props) = view.props() {
                    Self::unmount_props_events(event_manager, id, props, transaction);
                }
                if let Some(children) = view.children() {
                    for (index, child) in children.iter().enumerate() {
                        Self::unmount_view_internal(
                            event_manager,
                            &mut remove_ids,
                            id,
                            index,
                            child,
                            transaction,
                        );
                    }
                }

                Some(view.clone())
            } else {
                None
            }
        } else {
            None
        };

        for id in remove_ids {
            nodes_lock.remove(&id);
        }

        view
    }
}
