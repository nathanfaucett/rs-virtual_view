use std::sync::{Arc, Mutex, MutexGuard};

use super::super::{diff_children, diff_props_object, view_id, Children, Component, Props,
                   Transaction, Updater, View};
use super::Renderer;

/*
pub enum NodeInnerState {
    Mounting,
    Mounted,
    Updating,
    Updated,
    Unmounting,
    Unmounted,
}
*/

pub enum NodeInnerKind {
    View,
    Component {
        partial_states: Vec<Props>,
        node: Node,
        updater: Updater,
        state: Props,
        component: Arc<Component>,
    },
}

pub struct NodeInner {
    pub index: usize,
    pub depth: usize,
    pub id: String,
    pub renderer: Renderer,
    pub view: View,
    pub kind: NodeInnerKind,
}

impl NodeInner {
    #[inline]
    pub fn new(index: usize, depth: usize, id: String, renderer: &Renderer, view: View) -> Self {
        let kind = if let Some(component) = view.component().map(Clone::clone) {
            let state = component.initial_state(view.props().unwrap());
            let updater = Updater::new(id.clone(), depth, renderer.clone());
            let rendered_view = Self::render_component_view(&view, &state, &component, &updater);

            NodeInnerKind::Component {
                partial_states: Vec::new(),
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
    pub fn update_state<F>(&mut self, f: F)
    where
        F: Fn(&Props) -> Props,
    {
        match &mut self.kind {
            &mut NodeInnerKind::Component {
                ref state,
                ref mut partial_states,
                ..
            } => {
                partial_states.push(f(state));
            }
            _ => (),
        }
    }

    #[inline]
    pub fn next_state(&mut self) -> Props {
        match &mut self.kind {
            &mut NodeInnerKind::Component {
                ref mut partial_states,
                ..
            } => {
                let mut state = Props::new();

                for partial_state in partial_states.drain(..) {
                    for (k, v) in partial_state {
                        state.insert(k, v);
                    }
                }

                state
            }
            _ => Props::new(),
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
    fn rendered_view(&self) -> View {
        match &self.kind {
            &NodeInnerKind::Component { ref node, .. } => node.rendered_view(),
            &NodeInnerKind::View => self.view.clone(),
        }
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
                        ref mut children, ..
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
        let view = match &self.kind {
            &NodeInnerKind::Component {
                ref node,
                ref component,
                ..
            } => {
                let mut view = node.unmount(transaction);

                component.will_unmount();

                match &mut view {
                    &mut View::Data {
                        ref mut children, ..
                    } => for (index, child) in children.iter_mut().enumerate() {
                        let child_id = view_id(&self.id, child.key(), index);

                        if let Some(node) = self.renderer.nodes().get(child_id) {
                            *child = node.unmount(transaction);
                        }
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
        };

        self.renderer
            .nodes()
            .remove_at_depth(self.id.clone(), self.depth);

        view
    }

    #[inline]
    pub fn receive(&mut self, next_view: View, transaction: &mut Transaction) -> (bool, View) {
        let prev_view = self.view.clone();
        self.update(prev_view, next_view, transaction)
    }

    #[inline]
    pub fn update(
        &mut self,
        prev_view: View,
        next_view: View,
        transaction: &mut Transaction,
    ) -> (bool, View) {
        let next_state = self.next_state();

        match &mut self.kind {
            &mut NodeInnerKind::Component {
                ref mut state,
                ref updater,
                ref component,
                ref node,
                ..
            } => {
                let should_update = {
                    let empty_props = Props::new();
                    let empty_children = Children::new();

                    let next_props = next_view.props().unwrap_or(&empty_props);
                    let next_children = next_view.children().unwrap_or(&empty_children);

                    if &prev_view != &next_view {
                        component.receive_props(&next_state, next_props, next_children);
                    }

                    if component.should_update(
                        state,
                        prev_view.props().unwrap_or(&empty_props),
                        prev_view.children().unwrap_or(&empty_children),
                        &next_state,
                        next_props,
                        next_children,
                    ) {
                        component.will_update();
                        true
                    } else {
                        false
                    }
                };

                self.view = next_view;
                *state = next_state;

                if should_update {
                    node.receive(
                        Self::render_component_view(&self.view, state, component, updater),
                        transaction,
                    )
                } else {
                    (false, node.rendered_view())
                }
            }
            &mut NodeInnerKind::View => if prev_view != next_view {
                let mut view = next_view.clone_no_children();

                match &next_view {
                    &View::Data {
                        ref props,
                        ref children,
                        ..
                    } => {
                        let mut view_children = view.children_mut().unwrap();

                        let empty_children = Children::new();
                        let prev_children = prev_view.children().unwrap_or(&empty_children);
                        let next_children = diff_children(prev_children, children);

                        let empty_props = Props::new();
                        let prev_props = prev_view.props().unwrap_or(&empty_props);

                        if let Some(diff_props) = diff_props_object(prev_props, props) {
                            transaction.props(&self.id, prev_props.into(), diff_props.into());
                        }
                        /* TODO: defer update so event manager lock is released
                        self.renderer
                            .update_props_events(&self.id, prev_props, props, transaction);
                            */

                        for (index, next_view_option) in next_children.children.iter().enumerate() {
                            let prev_view_option = prev_children.get(index);

                            if let &Some(next_view) = next_view_option {
                                let next_view_id = view_id(&self.id, next_view.key(), index);

                                if let Some(prev_view) = prev_view_option {
                                    let prev_view_id = view_id(&self.id, prev_view.key(), index);

                                    assert!(
                                        prev_view_id == next_view_id,
                                        "prev and next id should not be different"
                                    );

                                    if let Some(node) = self.renderer.nodes().get(next_view_id) {
                                        let (updated, view) =
                                            node.receive(next_view.clone(), transaction);

                                        view_children.push(view);
                                    } else {
                                        // should be text view
                                        transaction.replace(
                                            &prev_view_id,
                                            prev_view.clone().into(),
                                            next_view.clone().into(),
                                        );
                                        view_children.push(next_view.clone());
                                    }
                                } else {
                                    let node = Node::new(
                                        index,
                                        0,
                                        next_view_id,
                                        &self.renderer,
                                        next_view.clone(),
                                    );
                                    let view = node.mount(transaction);
                                    view_children.push(view);
                                }
                            } else if let Some(prev_view) = prev_view_option {
                                let prev_view_id = view_id(&self.id, prev_view.key(), index);

                                if let Some(node) = self.renderer.nodes().get(prev_view_id) {
                                    let _ = node.unmount(transaction);
                                }
                            }
                        }
                    }
                    _ => (),
                }

                self.view = view.clone();

                (true, view)
            } else {
                (false, self.view.clone())
            },
        }
    }
}

#[derive(Clone)]
pub struct Node(Arc<Mutex<NodeInner>>);

impl Node {
    #[inline]
    pub fn new(index: usize, depth: usize, id: String, renderer: &Renderer, view: View) -> Self {
        let node = Node(Arc::new(Mutex::new(NodeInner::new(
            index,
            depth,
            id.clone(),
            renderer,
            view,
        ))));

        renderer.nodes().insert_at_depth(id, depth, node.clone());

        node
    }

    #[inline]
    pub fn lock(&self) -> MutexGuard<NodeInner> {
        self.0.lock().expect("failed to acquire node lock")
    }

    #[inline]
    fn rendered_view(&self) -> View {
        self.lock().rendered_view()
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
    pub fn receive(&self, next_view: View, transaction: &mut Transaction) -> (bool, View) {
        self.lock().receive(next_view, transaction)
    }
    #[inline]
    pub fn update(
        &self,
        prev_view: View,
        next_view: View,
        transaction: &mut Transaction,
    ) -> (bool, View) {
        self.lock().update(prev_view, next_view, transaction)
    }
}
