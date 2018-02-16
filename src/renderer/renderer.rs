use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::super::{EventManager, Props, Transaction, View};
use super::{Handler, Node, Nodes};

static ROOT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct RendererInner {
    root_id: String,
    root_index: usize,
    handler: Box<Handler>,
    nodes: Nodes,
    event_manager: EventManager,
}

#[derive(Clone)]
pub struct Renderer(Arc<RendererInner>);

impl Renderer {
    #[inline]
    pub fn new<H>(view: View, event_manager: EventManager, handler: H) -> Self
    where
        H: Handler,
    {
        let mut root_id = String::new();
        let root_index = ROOT_ID.fetch_add(1, Ordering::SeqCst);

        root_id.push('.');
        root_id.push_str(&root_index.to_string());

        let renderer = Renderer(Arc::new(RendererInner {
            root_index: root_index,
            root_id: root_id,
            handler: Box::new(handler),
            nodes: Nodes::new(),
            event_manager: event_manager,
        }));

        renderer.mount(view);

        renderer
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
    pub(super) fn transaction(&self, transaction: Transaction) {
        self.0.handler.handle(transaction);
    }

    #[inline]
    pub fn mount(&self, view: View) {
        let mut transaction = Transaction::new();
        let node = Node::new(self.0.root_index, 0, self.0.root_id.clone(), self, view);

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
        if let Some(node) = self.0.nodes.get_at_depth(id, depth) {
            let mut node_lock = node.lock();

            node_lock.update_state(f);

            let prev_view = node_lock.view.clone();
            let next_view = node_lock.view.clone();

            let mut transaction = Transaction::new();
            node_lock.update(prev_view, next_view, &mut transaction);
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
                if let Some(_) = v.function() {
                    if !next_props.has(k) {
                        transaction.remove_event(id, k);
                        event_manager.remove(id, k);
                    }
                }
            }
        }
    }
}
