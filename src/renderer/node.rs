use std::sync::{Arc, Mutex, MutexGuard};

use super::super::{diff_children, diff_props_object, view_id, Children, Component, Props,
                   Transaction, Updater, View};
use super::Renderer;

/*
pub enum NodeState {
    Mounting,
    Mounted,
    Updating,
    Updated,
    Unmounting,
    Unmounted,
}
*/

pub enum NodeKind {
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
    pub kind: NodeKind,
}

impl NodeInner {
    #[inline]
    pub fn new(index: usize, depth: usize, id: String, renderer: &Renderer, view: View) -> Self {
        let kind = if let Some(component) = view.component().map(Clone::clone) {
            let state = component.initial_state(view.props().unwrap());
            let updater = Updater::new(id.clone(), depth, renderer.clone());
            let rendered_view = Self::render_component_view(&view, &state, &component, &updater);

            NodeKind::Component {
                partial_states: Vec::new(),
                node: Node::new(index, depth + 1, id.clone(), renderer, rendered_view),
                updater: updater,
                state: state,
                component: component,
            }
        } else {
            NodeKind::View
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
            &mut NodeKind::Component {
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
            &mut NodeKind::Component {
                ref mut partial_states,
                ..
            } => {
                let mut state = Props::new();

                for partial_state in partial_states.drain(..) {
                    state.extend(partial_state);
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
            &NodeKind::Component { ref node, .. } => node.rendered_view(),
            &NodeKind::View => self.view.clone(),
        }
    }

    #[inline]
    pub fn mount(&mut self, transaction: &mut Transaction) -> View {
        match &self.kind {
            &NodeKind::Component {
                ref node,
                ref component,
                ..
            } => {
                let mut view = node.mount(transaction);

                component.will_mount();

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
            &NodeKind::View => {
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
            &NodeKind::Component {
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
            &NodeKind::View => {
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
    pub fn receive(&mut self, next_view: View, transaction: &mut Transaction) -> View {
        let prev_view = self.view.clone();
        self.update(prev_view, next_view, transaction)
    }

    #[inline]
    pub fn update(
        &mut self,
        prev_view: View,
        next_view: View,
        transaction: &mut Transaction,
    ) -> View {
        if prev_view.component() != next_view.component() {
            self.renderer
                .nodes()
                .remove_at_depth(self.id.clone(), self.depth);

            let node = Node::new(
                self.index,
                self.depth,
                self.id.clone(),
                &self.renderer,
                next_view,
            );
            let view = node.mount(transaction);
            transaction.replace(&self.id, self.rendered_view().into(), view.clone().into());
            view
        } else {
            let next_state = self.next_state();

            match &mut self.kind {
                &mut NodeKind::Component {
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
                        node.rendered_view()
                    }
                }
                &mut NodeKind::View => {
                    let mut view = next_view.clone_no_children();

                    match &next_view {
                        &View::Data {
                            props: ref next_props,
                            children: ref next_children,
                            ..
                        } => {
                            let mut view_children = view.children_mut().unwrap();

                            let empty_children = Children::new();
                            let prev_children = prev_view.children().unwrap_or(&empty_children);
                            let children_diff = diff_children(prev_children, next_children);

                            let empty_props = Props::new();
                            let prev_props = prev_view.props().unwrap_or(&empty_props);

                            if let Some(diff_props) = diff_props_object(prev_props, next_props) {
                                transaction.props(&self.id, prev_props.into(), diff_props.into());
                            }

                            self.renderer.update_props_events(
                                &self.id,
                                prev_props,
                                next_props,
                                transaction,
                            );

                            for (index, next_view_option) in
                                children_diff.children.iter().enumerate()
                            {
                                let prev_view_option = prev_children.get(index);

                                match next_view_option {
                                    &Some(next_view) => {
                                        let next_view_id =
                                            view_id(&self.id, next_view.key(), index);

                                        if let Some(prev_view) = prev_view_option {
                                            let prev_view_id =
                                                view_id(&self.id, prev_view.key(), index);

                                            assert!(
                                                prev_view_id == next_view_id,
                                                "prev and next id should not be different"
                                            );

                                            if let Some(node) =
                                                self.renderer.nodes().get(next_view_id)
                                            {
                                                let view =
                                                    node.receive(next_view.clone(), transaction);
                                                view_children.push(view);
                                            } else if &prev_view != &next_view {
                                                transaction.replace(
                                                    &prev_view_id,
                                                    prev_view.into(),
                                                    next_view.into(),
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
                                    }
                                    &None => {
                                        if let Some(prev_view) = prev_view_option {
                                            let prev_view_id =
                                                view_id(&self.id, prev_view.key(), index);

                                            if let Some(node) =
                                                self.renderer.nodes().get(prev_view_id)
                                            {
                                                let _ = node.unmount(transaction);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => (),
                    }

                    self.view = view.clone();

                    view
                }
            }
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
    pub fn receive(&self, next_view: View, transaction: &mut Transaction) -> View {
        self.lock().receive(next_view, transaction)
    }
}
