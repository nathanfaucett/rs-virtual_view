use std::sync::Arc;
use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};

use super::super::{diff_children, diff_props_object, parent_id, view_id, Children, Component,
                   Instance, Props, Transaction, Updater, View};
use super::Renderer;

pub enum NodeKind {
    View,
    Component {
        instance: Instance,
        node: Node,
        next_state: Option<Props>,
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
    pub parent_context: Props,
}

impl NodeInner {
    #[inline]
    pub fn new(
        index: usize,
        depth: usize,
        id: String,
        renderer: &Renderer,
        view: View,
        parent_context: &Props,
    ) -> Self {
        let kind = if let Some(component) = view.component().map(Clone::clone) {
            let mut context = component.context(view.props().unwrap());
            context = component.inherit_context(context, parent_context);

            let state = component.initial_state(view.props().unwrap());
            let updater = Updater::new(id.clone(), depth, renderer.clone());
            let instance = Instance::new(state, context, updater);

            let rendered_view = Self::render_component_view(&instance, &view, &component);

            NodeKind::Component {
                node: Node::new(
                    index,
                    depth + 1,
                    id.clone(),
                    renderer,
                    rendered_view,
                    &instance.context,
                ),
                instance: instance,
                next_state: None,
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
            parent_context: parent_context.clone(),
        }
    }

    #[inline]
    fn set_next_state<F>(&mut self, f: F)
    where
        F: Fn(&Props) -> Props,
    {
        match &mut self.kind {
            &mut NodeKind::Component {
                ref instance,
                ref mut next_state,
                ..
            } => {
                *next_state = Some(f(&instance.state));
            }
            _ => (),
        }
    }

    #[inline]
    pub fn update_state<F>(&mut self, f: F, transaction: &mut Transaction)
    where
        F: Fn(&Props) -> Props,
    {
        self.set_next_state(f);

        let prev_view = self.view.clone();
        let next_view = self.view.clone();

        self.update(prev_view, next_view, transaction);
    }

    #[inline]
    fn next_state(&mut self) -> Props {
        match &mut self.kind {
            &mut NodeKind::Component {
                ref mut next_state, ..
            } => next_state.take().unwrap_or_else(Props::new),
            _ => Props::new(),
        }
    }

    #[inline]
    fn render_component_view(instance: &Instance, view: &View, component: &Arc<Component>) -> View {
        let empty_props = Props::new();
        let empty_children = Children::new();

        let props = view.props().unwrap_or(&empty_props);
        let children = view.children().unwrap_or(&empty_children);

        component.render(instance, props, children)
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
                ref instance,
                ref node,
                ref component,
                ..
            } => {
                let mut view = node.mount(transaction);

                component.will_mount(instance);

                match &mut view {
                    &mut View::Data {
                        ref mut children, ..
                    } => for (index, child) in children.iter_mut().enumerate() {
                        if child.is_data() {
                            let child_id = view_id(&self.id, child.key(), index);
                            let node = Node::new(
                                index,
                                0,
                                child_id,
                                &self.renderer,
                                child.clone(),
                                &instance.context,
                            );
                            *child = node.mount(transaction);
                        }
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

                match &mut self.view {
                    &mut View::Data {
                        ref mut children, ..
                    } => for (index, child) in children.iter_mut().enumerate() {
                        if child.is_data() {
                            let child_id = view_id(&self.id, child.key(), index);
                            let node = Node::new(
                                index,
                                0,
                                child_id,
                                &self.renderer,
                                child.clone(),
                                &self.parent_context,
                            );
                            *child = node.mount(transaction);
                        }
                    },
                    _ => (),
                }

                self.view.clone()
            }
        }
    }

    #[inline]
    pub fn unmount(&mut self, transaction: &mut Transaction) -> View {
        let view = match &self.kind {
            &NodeKind::Component {
                ref instance,
                ref node,
                ref component,
                ..
            } => {
                let mut view = node.unmount(transaction);

                component.will_unmount(instance);

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

                match &mut self.view {
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
        if Self::should_update(&prev_view, &next_view) {
            self.internal_update(prev_view, next_view, transaction)
        } else {
            self.unmount(transaction);

            let node = Node::new(
                self.index,
                self.depth,
                view_id(&parent_id(&self.id), next_view.key(), self.index),
                &self.renderer,
                next_view,
                &self.parent_context,
            );
            let view = node.mount(transaction);
            transaction.replace(&self.id, self.rendered_view().into(), view.clone().into());
            view
        }
    }

    #[inline]
    pub fn should_update(prev_view: &View, next_view: &View) -> bool {
        match prev_view {
            &View::Data {
                kind: ref prev_kind,
                key: ref prev_key,
                ..
            } => match next_view {
                &View::Data {
                    kind: ref next_kind,
                    key: ref next_key,
                    ..
                } => prev_kind == next_kind && prev_key == next_key,
                &View::Text(_) => false,
            },
            &View::Text(_) => match next_view {
                &View::Data { .. } => false,
                &View::Text(_) => true,
            },
        }
    }

    #[inline]
    pub fn internal_update(
        &mut self,
        prev_view: View,
        next_view: View,
        transaction: &mut Transaction,
    ) -> View {
        let next_state = self.next_state();

        match &mut self.kind {
            &mut NodeKind::Component {
                ref mut instance,
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
                        component.receive_props(instance, &next_state, next_props, next_children);
                    }

                    if component.should_update(
                        &instance.state,
                        prev_view.props().unwrap_or(&empty_props),
                        prev_view.children().unwrap_or(&empty_children),
                        &next_state,
                        next_props,
                        next_children,
                    ) {
                        component.will_update(instance);
                        true
                    } else {
                        false
                    }
                };

                self.view = next_view;
                instance.state = next_state;

                if should_update {
                    node.receive(
                        Self::render_component_view(instance, &self.view, component),
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

                        for (index, next_view_option) in children_diff.children.iter().enumerate() {
                            let prev_view_option = prev_children.get(index);

                            if let &Some(next_view) = next_view_option {
                                let next_view_id = view_id(&self.id, next_view.key(), index);

                                if let Some(prev_view) = prev_view_option {
                                    let prev_view_id = view_id(&self.id, prev_view.key(), index);

                                    if let Some(node) = self.renderer.nodes().get(prev_view_id) {
                                        let view = node.receive(next_view.clone(), transaction);
                                        view_children.push(view);
                                    } else {
                                        if &prev_view != &next_view {
                                            transaction.replace(
                                                &next_view_id,
                                                prev_view.into(),
                                                next_view.into(),
                                            );
                                        }
                                        view_children.push(next_view.clone());
                                    }
                                } else {
                                    let node = Node::new(
                                        index,
                                        0,
                                        next_view_id.clone(),
                                        &self.renderer,
                                        next_view.clone(),
                                        &self.parent_context,
                                    );
                                    let view = node.mount(transaction);
                                    transaction.insert(
                                        &self.id,
                                        &next_view_id,
                                        index,
                                        view.clone().into(),
                                    );
                                    view_children.push(view);
                                }
                            } else if let Some(prev_view) = prev_view_option {
                                let prev_view_id = view_id(&self.id, prev_view.key(), index);

                                if let Some(node) = self.renderer.nodes().get(prev_view_id.clone())
                                {
                                    let view = node.unmount(transaction);
                                    transaction.remove(&prev_view_id, view.into());
                                }
                            }
                        }

                        if let Some(diff_props) = diff_props_object(prev_props, next_props) {
                            transaction.props(&self.id, prev_props.into(), diff_props.into());
                        }

                        let order = children_diff.into_order();
                        if !order.is_empty() {
                            transaction.order(&self.id, order);
                        }

                        self.renderer.update_props_events(
                            &self.id,
                            prev_props,
                            next_props,
                            transaction,
                        );
                    }
                    _ => (),
                }

                self.view = view.clone();

                view
            }
        }
    }
}

#[derive(Clone)]
pub struct Node(Rc<RefCell<NodeInner>>);

impl Node {
    #[inline]
    pub fn new(
        index: usize,
        depth: usize,
        id: String,
        renderer: &Renderer,
        view: View,
        parent_context: &Props,
    ) -> Self {
        let node = Node(Rc::new(RefCell::new(NodeInner::new(
            index,
            depth,
            id.clone(),
            renderer,
            view,
            parent_context,
        ))));

        renderer.nodes().insert_at_depth(id, depth, node.clone());

        node
    }

    #[inline]
    pub fn as_ref(&self) -> Ref<NodeInner> {
        self.0.borrow()
    }
    #[inline]
    pub fn as_mut(&self) -> RefMut<NodeInner> {
        self.0.borrow_mut()
    }

    #[inline]
    pub fn rendered_view(&self) -> View {
        self.as_ref().rendered_view()
    }

    #[inline]
    pub fn mount(&self, transaction: &mut Transaction) -> View {
        self.as_mut().mount(transaction)
    }
    #[inline]
    pub fn unmount(&self, transaction: &mut Transaction) -> View {
        self.as_mut().unmount(transaction)
    }
    #[inline]
    pub fn receive(&self, next_view: View, transaction: &mut Transaction) -> View {
        self.as_mut().receive(next_view, transaction)
    }
}
