use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};

use fnv::FnvHashSet;

use super::super::{view_id, EventManager, Props, Transaction, View};
use super::{ComponentNode, Node, Nodes, Updater, ViewNode};

static ROOT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct TreeInner {
    root_index: usize,
    root_id: String,
    sender: Sender<Transaction>,
    event_manager: EventManager,
    nodes: Nodes,
}

#[derive(Clone)]
pub struct Tree {
    inner: Arc<TreeInner>,
}

impl Tree {
    #[inline]
    pub fn new(view: View) -> (Self, Receiver<Transaction>) {
        let (sender, receiver) = channel();
        let mut root_id = String::new();
        let root_index = ROOT_ID.fetch_add(1, Ordering::SeqCst);

        root_id.push('.');
        root_id.push_str(&root_index.to_string());

        let tree = Tree {
            inner: Arc::new(TreeInner {
                root_index: root_index,
                root_id: root_id,
                sender: sender,
                event_manager: EventManager::new(),
                nodes: Nodes::new(),
            }),
        };

        tree.mount(view);

        (tree, receiver)
    }

    #[inline(always)]
    pub fn root_index(&self) -> usize {
        self.inner.root_index
    }
    #[inline(always)]
    pub fn root_id(&self) -> &String {
        &self.inner.root_id
    }
    #[inline(always)]
    pub fn nodes(&self) -> &Nodes {
        &self.inner.nodes
    }
    #[inline(always)]
    pub fn event_manager(&self) -> &EventManager {
        &self.inner.event_manager
    }

    #[inline(always)]
    fn transaction(&self, transaction: Transaction) {
        let _ = self.inner
            .sender
            .send(transaction)
            .expect("failed to send transaction");
    }

    #[inline]
    pub fn update<F>(&self, _id: &str, _f: F)
    where
        F: Fn(&mut Props),
    {

    }

    #[inline]
    pub fn mount(&self, view: View) {
        let mut transaction = Transaction::new();

        if self.inner.nodes.lock().len() != 0 {
            self.unmount();
        }

        let rendered_view = self.mount_view(self.inner.root_id.clone(), view, &mut transaction);
        transaction.mount(&self.inner.root_id, rendered_view.into());

        self.transaction(transaction);
    }

    #[inline]
    pub fn unmount(&self) {
        let mut transaction = Transaction::new();

        if let Some(rendered_view) = self.unmount_view(&self.inner.root_id, &mut transaction) {
            transaction.unmount(&self.inner.root_id, rendered_view.into());
            self.transaction(transaction);
        }
    }

    #[inline]
    pub(super) fn mount_props_events(
        &self,
        id: &str,
        props: &Props,
        transaction: &mut Transaction,
    ) {
        let event_manager = &self.inner.event_manager;

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
    fn unmount_props_events(&self, id: &str, props: &Props, transaction: &mut Transaction) {
        let event_manager = &self.inner.event_manager;

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
    pub(super) fn mount_view(&self, id: String, view: View, transaction: &mut Transaction) -> View {
        let mut node: Box<Node> = if let Some(component) = view.component().map(Clone::clone) {
            Box::new(ComponentNode::new(
                id.clone(),
                "".into(),
                view,
                component,
                Updater::new(id.clone(), self.clone()),
            ))
        } else {
            Box::new(ViewNode::new(id.clone(), "".into(), view))
        };

        let rendered_view = node.mount(self, transaction);
        self.inner.nodes.insert(id, node);

        rendered_view
    }

    #[inline]
    fn unmount_view_internal(
        &self,
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

                self.unmount_props_events(&child_id, props, transaction);

                for (index, child) in children.iter().enumerate() {
                    self.unmount_view_internal(remove_ids, &child_id, index, child, transaction);
                }

                remove_ids.insert(child_id);
            }
            _ => (),
        }
    }

    #[inline]
    fn unmount_view(&self, id: &String, transaction: &mut Transaction) -> Option<View> {
        let mut nodes_lock = self.inner.nodes.lock();

        let mut remove_ids = FnvHashSet::default();

        let view = if let Some(node) = nodes_lock.get(id) {
            remove_ids.insert(id.clone());

            if let Some(view) = node.last_rendered_view() {
                match view {
                    &View::Data {
                        ref props,
                        ref children,
                        ..
                    } => {
                        self.unmount_props_events(id, props, transaction);

                        for (index, child) in children.iter().enumerate() {
                            self.unmount_view_internal(
                                &mut remove_ids,
                                id,
                                index,
                                child,
                                transaction,
                            );
                        }
                    }
                    _ => (),
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
