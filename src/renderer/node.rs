use std::sync::{Arc, Mutex, MutexGuard};
use std::any::Any;

use super::super::{view_id, Children, Component, Props, Transaction, Updater, View};
use super::Renderer;

pub enum NodeInnerKind {
    View,
    Component {
        node: Node,
        updater: Updater,
        state: Props,
        component: Arc<Component>,
    },
}

pub struct NodeInner {
    index: usize,
    depth: usize,
    id: String,
    renderer: Renderer,
    view: View,
    kind: NodeInnerKind,
}

impl NodeInner {
    #[inline]
    pub fn new(index: usize, depth: usize, id: String, renderer: &Renderer, view: View) -> Self {
        let kind = if let Some(component) = view.component().map(Clone::clone) {
            let state = component.initial_state(view.props().unwrap());
            let updater = Updater::new(id.clone(), depth, renderer.clone());
            let rendered_view = Self::render_component_view(&view, &state, &component, &updater);

            NodeInnerKind::Component {
                node: Node::new(index, depth + 1, id.clone(), renderer, rendered_view),
                updater: updater,
                state: state,
                component: component,
            }
        } else {
            NodeInnerKind::View
        };

        NodeInner {
            index: index,
            depth: depth,
            id: id,
            renderer: renderer.clone(),
            view: view,
            kind: kind,
        }
    }

    #[inline]
    fn render_component_view(
        view: &View,
        state: &Props,
        component: &Arc<Component>,
        updater: &Updater,
    ) -> View {
        let empty_props = Props::new();
        let empty_children = Children::new();

        let props = view.props().unwrap_or(&empty_props);
        let children = view.children().unwrap_or(&empty_children);

        component.render(updater, state, props, children)
    }

    #[inline]
    pub fn mount(&mut self, transaction: &mut Transaction) -> View {
        match &self.kind {
            &NodeInnerKind::Component {
                ref updater,
                ref node,
                ref component,
                ..
            } => {
                let mut view = node.mount(transaction);

                component.will_mount(updater);

                match &mut view {
                    &mut View::Data {
                        ref mut children,
                        ref props,
                        ..
                    } => for (index, child) in children.iter_mut().enumerate() {
                        let child_id = view_id(&self.id, child.key(), index);
                        let node = Node::new(index, 0, child_id, &self.renderer, child.clone());
                        *child = node.mount(transaction);
                    },
                    _ => (),
                }

                view
            }
            &NodeInnerKind::View => {
                if let Some(props) = self.view.props() {
                    self.renderer
                        .mount_props_events(&self.id, props, transaction);
                }
                self.view.clone()
            }
        }
    }

    #[inline]
    pub fn unmount(&mut self, transaction: &mut Transaction) -> View {
        match &self.kind {
            &NodeInnerKind::Component {
                ref updater,
                ref node,
                ref component,
                ..
            } => {
                let mut view = node.unmount(transaction);

                component.will_unmount();

                match &mut view {
                    &mut View::Data {
                        ref mut children,
                        ref props,
                        ..
                    } => for (index, child) in children.iter_mut().enumerate() {
                        let child_id = view_id(&self.id, child.key(), index);
                        let node = Node::new(index, 0, child_id, &self.renderer, child.clone());
                        *child = node.unmount(transaction);
                    },
                    _ => (),
                }

                view
            }
            &NodeInnerKind::View => {
                if let Some(props) = self.view.props() {
                    self.renderer
                        .unmount_props_events(&self.id, props, transaction);
                }
                self.view.clone()
            }
        }
    }

    #[inline]
    pub fn receive(&mut self, next_view: View) {}

    #[inline]
    pub fn update(&mut self, prev_view: View, next_view: View) {}
}

#[derive(Clone)]
pub struct Node(Arc<Mutex<NodeInner>>);

impl Node {
    #[inline]
    pub fn new(index: usize, depth: usize, id: String, renderer: &Renderer, view: View) -> Self {
        Node(Arc::new(Mutex::new(NodeInner::new(
            index,
            depth,
            id,
            renderer,
            view,
        ))))
    }

    #[inline]
    pub fn lock(&self) -> MutexGuard<NodeInner> {
        self.0.lock().expect("failed to acquire node lock")
    }

    #[inline]
    pub fn mount(&self, transaction: &mut Transaction) -> View {
        self.lock().mount(transaction)
    }
    #[inline]
    pub fn unmount(&self, transaction: &mut Transaction) -> View {
        self.lock().unmount(transaction)
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
