use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::super::{EventManager, Props, Transaction, View};
use super::{Handler, Message, Node, Nodes, Queue};

static ROOT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct RendererInner {
    root_id: String,
    root_index: usize,
    handler: Box<Handler>,
    nodes: Nodes,
    event_manager: EventManager,
    queue: Queue,
    processing: AtomicBool,
}

#[derive(Clone)]
pub struct Renderer(Arc<RendererInner>);

unsafe impl Send for Renderer {}
unsafe impl Sync for Renderer {}

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
            queue: Queue::new(),
            processing: AtomicBool::new(false),
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

    #[inline]
    pub fn processing(&self) -> bool {
        !self.0
            .processing
            .compare_and_swap(false, true, Ordering::SeqCst)
    }
    #[inline]
    fn finish_processing(&self) {
        self.0.processing.store(false, Ordering::SeqCst);
        self.process_queue();
    }

    #[inline(always)]
    fn process_queue(&self) {
        if self.processing() {
            if let Some(message) = self.0.queue.pop() {
                match message {
                    Message::Mount(view) => self.internal_mount(view),
                    Message::Update(id, depth, f) => self.internal_update(id, depth, f),
                    Message::Unmount => self.internal_unmount(),
                }
            } else {
                self.0.processing.store(false, Ordering::SeqCst);
            }
        }
    }

    #[inline(always)]
    fn handle_transaction(&self, transaction: Transaction) {
        self.0.handler.handle(transaction);
    }

    #[inline]
    pub fn mount(&self, view: View) {
        if !self.0.nodes.is_empty() {
            self.unmount();
        }
        self.0.queue.push_mount(view);
        self.process_queue();
    }

    #[inline]
    pub fn unmount(&self) {
        self.0.queue.push_unmount();
        self.process_queue();
    }

    #[inline]
    pub(super) fn update<F>(&self, id: String, depth: usize, f: F)
    where
        F: 'static + Send + Fn(&Props) -> Props,
    {
        self.0.queue.push_update(id, depth, f);
        self.process_queue();
    }

    #[inline]
    fn internal_mount(&self, view: View) {
        let mut transaction = Transaction::new();
        let node = Node::new(
            self.0.root_index,
            0,
            self.0.root_id.clone(),
            self,
            view,
            &Props::new(),
        );

        let view = node.mount(&mut transaction);
        transaction.mount(&self.0.root_id, view.into());

        self.handle_transaction(transaction);

        self.finish_processing();
    }

    #[inline]
    fn internal_unmount(&self) {
        let mut transaction = Transaction::new();

        let unmounted_view = if let Some(node) = self.0.nodes.get(self.0.root_id.clone()) {
            Some(node.unmount(&mut transaction))
        } else {
            None
        };

        if let Some(view) = unmounted_view {
            transaction.unmount(&self.0.root_id, view.into());
            self.handle_transaction(transaction);
        }

        self.finish_processing();
    }

    #[inline]
    fn internal_update(&self, id: String, depth: usize, f: Box<Fn(&Props) -> Props + Send>) {
        if let Some(node) = self.0.nodes.get_at_depth(id, depth) {
            let mut transaction = Transaction::new();

            node.as_mut().update_state(&*f, &mut transaction);

            if !transaction.is_empty() {
                self.handle_transaction(transaction);
            }
        }

        self.finish_processing();
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
