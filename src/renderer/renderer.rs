use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};

use super::super::{EventManager, Transaction, View};
use super::{Handler, Node};

static ROOT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct RendererInner {
    root_id: String,
    root_index: usize,
    sender: Sender<Transaction>,
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

    #[inline(always)]
    fn transaction(&self, transaction: Transaction) {
        let _ = self.0
            .sender
            .send(transaction)
            .expect("failed to send transaction");
    }

    #[inline]
    fn mount(&self, view: View) {
        let mut transaction = Transaction::new();
        let node = Node::new(self, view);

        let view = node.mount(&mut transaction);
        transaction.mount(&self.0.root_id, view.into());

        self.transaction(transaction);
    }
}
