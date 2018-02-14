use std::sync::{Arc, Mutex, MutexGuard};

use super::super::{Children, Component, Props, Transaction, View};
use super::{Tree, Updater};

pub enum NodeKind {
    View,
    Component {
        state: Props,
        updater: Updater,
        component: Arc<Component>,
    },
}

pub struct NodeInner {
    pub(super) depth: usize,
    pub(super) index: usize,
    pub(super) view: View,
    pub(super) rendered_view: View,
    pub(super) kind: NodeKind,
}

#[derive(Clone)]
pub struct Node(Arc<Mutex<NodeInner>>);

impl Node {
    #[inline]
    pub fn new_view(depth: usize, index: usize, view: View) -> Self {
        Node(Arc::new(Mutex::new(NodeInner {
            depth: depth,
            index: index,
            view: view,
            rendered_view: View::Text(String::new()),
            kind: NodeKind::View,
        })))
    }
    #[inline]
    pub fn new_component(
        depth: usize,
        index: usize,
        view: View,
        updater: Updater,
        component: Arc<Component>,
    ) -> Self {
        let state = component.initial_state(view.props().unwrap());

        Node(Arc::new(Mutex::new(NodeInner {
            depth: depth,
            index: index,
            view: view,
            rendered_view: View::Text(String::new()),
            kind: NodeKind::Component {
                state: state,
                updater: updater,
                component: component,
            },
        })))
    }
    #[inline]
    pub fn lock(&self) -> MutexGuard<NodeInner> {
        self.0.lock().expect("failed to acquire node lock")
    }
}
