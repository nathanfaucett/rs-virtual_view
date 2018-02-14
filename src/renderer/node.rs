use std::sync::{Arc, Mutex, MutexGuard};

use super::super::{Transaction, View};
use super::{Handler, Renderer};

pub struct Node(Arc<Mutex<Box<Handler>>>);

impl Node {
    #[inline]
    pub fn new(renderer: &Renderer, view: View) -> Self {
        Node(Arc::new(Mutex::new(Handler::new(renderer, view))))
    }

    #[inline]
    pub fn lock(&self) -> MutexGuard<Box<Handler>> {
        self.0.lock().expect("failed to acquire node lock")
    }

    #[inline]
    pub fn mount(&self, transaction: &mut Transaction) -> View {
        self.lock().mount(transaction)
    }
    #[inline]
    pub fn receive(&self, next_view: View) {
        self.lock().receive(next_view)
    }
    #[inline]
    pub fn update(&self, prev_view: View, next_view: View) {
        self.lock().update(prev_view, next_view)
    }
}
