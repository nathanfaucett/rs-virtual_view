use std::sync::{Arc, Mutex, MutexGuard};
use std::collections::LinkedList;

use super::super::{Props, View};

pub enum Message {
    Mount(View),
    Update(String, usize, Box<Fn(&Props) -> Props + Send>),
    Unmount,
}

unsafe impl Sync for Message {}
unsafe impl Send for Message {}

#[derive(Clone)]
pub struct Queue(Arc<Mutex<LinkedList<Message>>>);

impl Queue {
    #[inline]
    pub fn new() -> Self {
        Queue(Arc::new(Mutex::new(LinkedList::new())))
    }

    #[inline]
    pub fn lock(&self) -> MutexGuard<LinkedList<Message>> {
        self.0.lock().expect("failed to acquire queue lock")
    }

    #[inline]
    fn push(&self, message: Message) {
        self.lock().push_front(message);
    }

    #[inline]
    pub fn push_mount(&self, view: View) {
        self.push(Message::Mount(view));
    }
    #[inline]
    pub fn push_update<F>(&self, id: String, depth: usize, f: F)
    where
        F: 'static + Send + Fn(&Props) -> Props,
    {
        self.push(Message::Update(id, depth, Box::new(f)))
    }
    #[inline]
    pub fn push_unmount(&self) {
        self.push(Message::Unmount);
    }

    #[inline]
    pub fn pop(&self) -> Option<Message> {
        self.lock().pop_back()
    }
}
