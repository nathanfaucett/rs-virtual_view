use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};

use super::super::{EventManager, Props, Transaction, View};
use super::{Node, Nodes};

static ROOT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct RendererInner {
    root_id: String,
    root_index: usize,
    sender: Sender<Transaction>,
    nodes: Nodes,
    event_manager: EventManager,
}

#[derive(Clone)]
pub struct Renderer(Arc<RendererInner>);

impl Renderer {
    #[inline]
    pub fn new(view: View) -> (Self, Receiver<Transaction>) {
        let (sender, receiver) = channel();
        let mut root_id = String::new();
        let root_index = ROOT_ID.fetch_add(1, Ordering::SeqCst);

        root_id.push('.');
        root_id.push_str(&root_index.to_string());

        let renderer = Renderer(Arc::new(RendererInner {
            root_index: root_index,
            root_id: root_id,
            sender: sender,
            nodes: Nodes::new(),
            event_manager: EventManager::new(),
        }));

        renderer.mount(view);

        (renderer, receiver)
    }

    #[inline]
    pub fn root_id(&self) -> &String {
        &self.0.root_id
    }
    #[inline]
    pub fn root_index(&self) -> usize {
        self.0.root_index
    }
    #[inline]
    pub fn event_manager(&self) -> &EventManager {
        &self.0.event_manager
    }
    #[inline]
    pub(super) fn nodes(&self) -> &Nodes {
        &self.0.nodes
    }

    #[inline(always)]
    fn transaction(&self, transaction: Transaction) {
        let _ = self.0
            .sender
            .send(transaction)
            .expect("failed to send transaction");
    }

    #[inline]
    pub fn mount(&self, view: View) {
        let mut transaction = Transaction::new();
        let node = Node::new(self.0.root_index, 0, self.0.root_id.clone(), self, view);

        self.0.nodes.insert(self.0.root_id.clone(), node.clone());

        let view = node.mount(&mut transaction);
        transaction.mount(&self.0.root_id, view.into());

        self.transaction(transaction);
    }

    #[inline]
    pub fn unmount(&self) {
        let mut transaction = Transaction::new();

        let unmounted_view = if let Some(node) = self.0.nodes.get(self.0.root_id.clone()) {
            Some(node.unmount(&mut transaction))
        } else {
            None
        };

        if let Some(view) = unmounted_view {
            transaction.unmount(&self.0.root_id, view.into());
            self.transaction(transaction);
        }
    }

    #[inline]
    pub(super) fn update<F>(&self, id: String, depth: usize, f: F)
    where
        F: Fn(&Props) -> Props,
    {
        let mut transaction = Transaction::new();
        f(&Props::new());
        self.transaction(transaction);
    }

    #[inline]
    pub(super) fn mount_props_events(
        &self,
        id: &str,
        props: &Props,
        transaction: &mut Transaction,
    ) {
        let mut event_manager = self.0.event_manager.write();

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
        &self,
        id: &str,
        props: &Props,
        transaction: &mut Transaction,
    ) {
        let mut event_manager = self.0.event_manager.write();

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
    pub(super) fn update_props_events(
        &self,
        id: &str,
        prev_props: &Props,
        next_props: &Props,
        transaction: &mut Transaction,
    ) {
        let mut event_manager = self.0.event_manager.write();

        for (k, v) in next_props {
            if k.starts_with("on") {
                if let Some(f) = v.function() {
                    if !prev_props.has(k) {
                        transaction.add_event(id, k);
                        event_manager.add(id, k, f.clone());
                    }
                }
            }
        }
        for (k, v) in prev_props {
            if k.starts_with("on") {
                if let Some(f) = v.function() {
                    if !next_props.has(k) {
                        transaction.remove_event(id, k);
                        event_manager.remove(id, k);
                    }
                }
            }
        }
    }
}
