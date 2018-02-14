use std::sync::{Arc, MutexGuard};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};

use fnv::FnvHashSet;

use super::super::{diff_children, diff_props_object, parent_id, view_id, Children, EventManager,
                   Order, Props, Transaction, View};
use super::{Node, NodeInner, NodeKind, Nodes, Updater};

static ROOT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct TreeInner {
    root_index: usize,
    root_id: String,
    sender: Sender<Transaction>,
    event_manager: EventManager,
    nodes: Nodes,
}

#[derive(Clone)]
pub struct Tree(Arc<TreeInner>);

impl Tree {
    #[inline]
    pub fn new(view: View) -> (Self, Receiver<Transaction>) {
        let (sender, receiver) = channel();
        let mut root_id = String::new();
        let root_index = ROOT_ID.fetch_add(1, Ordering::SeqCst);

        root_id.push('.');
        root_id.push_str(&root_index.to_string());

        let tree = Tree(Arc::new(TreeInner {
            root_index: root_index,
            root_id: root_id,
            sender: sender,
            event_manager: EventManager::new(),
            nodes: Nodes::new(),
        }));

        tree.mount(view);

        (tree, receiver)
    }

    #[inline(always)]
    pub fn root_index(&self) -> usize {
        self.0.root_index
    }
    #[inline(always)]
    pub fn root_id(&self) -> &String {
        &self.0.root_id
    }
    #[inline(always)]
    pub fn nodes(&self) -> &Nodes {
        &self.0.nodes
    }
    #[inline(always)]
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
    fn render_view_with_state(node_inner: &NodeInner, state: &Props) -> View {
        let empty_props = Props::new();
        let empty_children = Children::new();

        let props = node_inner.view.props().unwrap_or(&empty_props);
        let children = node_inner.view.children().unwrap_or(&empty_children);

        match node_inner.kind {
            NodeKind::Component {
                ref updater,
                ref component,
                ..
            } => component.render(updater, state, props, children),
            NodeKind::View => node_inner.view.clone(),
        }
    }
    #[inline]
    fn render_view(node_inner: &NodeInner) -> View {
        let empty_props = Props::new();
        let empty_children = Children::new();

        let props = node_inner.view.props().unwrap_or(&empty_props);
        let children = node_inner.view.children().unwrap_or(&empty_children);

        match node_inner.kind {
            NodeKind::Component {
                ref state,
                ref updater,
                ref component,
            } => component.render(updater, state, props, children),
            NodeKind::View => node_inner.view.clone(),
        }
    }

    #[inline]
    pub fn mount(&self, view: View) {
        let mut transaction = Transaction::new();

        if self.0.nodes.read().len() != 0 {
            self.unmount();
        }

        let rendered_view = self.mount_view(self.0.root_id.clone(), 0, 0, view, &mut transaction);
        transaction.mount(&self.0.root_id, rendered_view.into());

        self.transaction(transaction);
    }

    #[inline]
    fn mount_props_events(&self, id: &str, props: &Props, transaction: &mut Transaction) {
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
    fn mount_view(
        &self,
        id: String,
        depth: usize,
        index: usize,
        view: View,
        transaction: &mut Transaction,
    ) -> View {
        let mut node = if let Some(component) = view.component().map(Clone::clone) {
            Node::new_component(
                depth,
                index,
                view,
                Updater::new(id.clone(), depth, self.clone()),
                component,
            )
        } else {
            Node::new_view(depth, index, view)
        };

        let rendered_view = {
            let node_lock = node.lock();
            let mut rendered_view = Self::render_view(&node_lock);
            self.mount_view_internal(node_lock, &id, depth, index, rendered_view, transaction)
        };

        self.0.nodes.insert(id, depth, node);

        rendered_view
    }

    #[inline]
    fn mount_view_internal(
        &self,
        mut node_lock: MutexGuard<NodeInner>,
        id: &str,
        depth: usize,
        index: usize,
        mut rendered_view: View,
        transaction: &mut Transaction,
    ) -> View {
        match &mut rendered_view {
            &mut View::Data {
                ref props,
                ref mut children,
                ..
            } => {
                self.mount_props_events(id, props, transaction);

                for (index, child) in children.iter_mut().enumerate() {
                    if child.is_data() {
                        let child_id = view_id(id, child.key(), index);
                        *child = self.mount_view(child_id, 0, index, child.clone(), transaction);
                    }
                }
            }
            _ => (),
        }

        node_lock.rendered_view = rendered_view.clone();

        rendered_view
    }

    #[inline]
    pub fn unmount(&self) {
        let mut transaction = Transaction::new();

        if let Some(rendered_view) = self.unmount_view(self.0.root_id.clone(), 0, &mut transaction)
        {
            transaction.unmount(&self.0.root_id, rendered_view.into());
            self.transaction(transaction);
        }
    }

    #[inline]
    fn unmount_props_events(&self, id: &str, props: &Props, transaction: &mut Transaction) {
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
    fn unmount_view(
        &self,
        id: String,
        depth: usize,
        transaction: &mut Transaction,
    ) -> Option<View> {
        if let Some(node) = self.0.nodes.remove_at_depth(id.clone(), depth) {
            let node_lock = node.lock();
            let rendered_view = node_lock.rendered_view.clone();

            match &rendered_view {
                &View::Data {
                    ref props,
                    ref children,
                    ..
                } => {
                    self.unmount_props_events(&id, props, transaction);

                    for (index, child) in children.iter().enumerate() {
                        self.unmount_view(id.clone(), 0, transaction);
                    }
                }
                _ => (),
            }

            Some(rendered_view)
        } else {
            None
        }
    }

    #[inline]
    pub fn update<F>(&self, id: String, depth: usize, f: F)
    where
        F: Fn(&Props) -> Props,
    {
        if let Some(node) = self.0.nodes.get_at_depth(id.clone(), depth) {
            let next_state = match &mut node.lock().kind {
                &mut NodeKind::Component { ref mut state, .. } => Some(f(state)),
                _ => None,
            };

            if let Some(next_state) = next_state {
                self.update_view(id, depth, node, next_state);
            }
        }
    }

    #[inline]
    fn update_view(&self, id: String, depth: usize, node: Node, next_state: Props) {
        let mut node_lock = node.lock();
        let index = node_lock.index;

        let mut next_view = Self::render_view_with_state(&node_lock, &next_state);

        let mut transaction = Transaction::new();
        let parent_id = parent_id(&id);

        self.update_view_internal(node_lock, &parent_id, index, next_view, &mut transaction);

        self.transaction(transaction);
    }

    #[inline]
    fn update_view_internal(
        &self,
        mut node_lock: MutexGuard<NodeInner>,
        parent_id: &str,
        index: usize,
        next_view: View,
        transaction: &mut Transaction,
    ) -> View {
        let child_id = view_id(parent_id, next_view.key(), index);

        match &node_lock.rendered_view {
            &View::Text(ref prev_text) => match &next_view {
                &View::Text(ref next_text) => if prev_text != next_text {
                    println!("text view content updated with new text");
                },
                &View::Data { .. } => {
                    println!("text replaced by data view");
                }
            },
            &View::Data {
                kind: ref prev_kind,
                key: ref prev_key,
                props: ref prev_props,
                children: ref prev_children,
            } => match &next_view {
                &View::Text(ref next_text) => {
                    println!("data view replaced with text view");
                }
                &View::Data {
                    kind: ref next_kind,
                    key: ref next_key,
                    props: ref next_props,
                    children: ref next_children,
                } => if prev_kind == next_kind && prev_key == next_key {
                    if next_kind.is_component() {
                        println!("update data view subcomponent");
                    } else {
                        let children = diff_children(prev_children, next_children);

                        for (index, next_child) in children.children.iter().enumerate() {
                            let prev_child = prev_children.get(index);
                        }
                    }
                } else {
                    println!("replace data view");
                },
            },
        }

        next_view

        /*
        match next_view_option {
            Some(next_view) => match prev_view_option {
                Some(prev_view) => match prev_view {
                    &View::Text(ref prev_text) => match next_view {
                        &View::Text(ref next_text) => if prev_text != next_text {
                            let id = view_id(parent_id, prev_view.key(), index);
                            transaction.replace(&id, prev_view.into(), next_view.into());
                        },
                        &View::Data {
                            props: ref next_props,
                            ..
                        } => {
                            let id = view_id(parent_id, prev_view.key(), index);
                            self.mount_props_events(&id, next_props, transaction);
                            transaction.replace(&id, prev_view.into(), next_view.into());
                        }
                    },
                    &View::Data {
                        key: ref prev_key,
                        props: ref prev_props,
                        children: ref prev_children,
                        ..
                    } => match next_view {
                        &View::Text(_) => {
                            let id = view_id(parent_id, prev_view.key(), index);
                            self.unmount_props_events(&id, prev_props, transaction);
                            transaction.replace(&id, prev_view.into(), next_view.into());
                        }
                        &View::Data {
                            key: ref next_key,
                            props: ref next_props,
                            children: ref next_children,
                            ..
                        } => if prev_key == next_key {
                            let children = diff_children(prev_children, next_children);
                            let id = view_id(parent_id, next_key.as_ref(), index);

                            for i in 0..children.children.len() {
                                self.update_view_internal(
                                    &id,
                                    i,
                                    prev_children.get(i),
                                    children.children[i],
                                    transaction,
                                );
                            }

                            if children.removes.len() != 0 || children.inserts.len() != 0 {
                                transaction.order(
                                    &id,
                                    Order::new(
                                        children
                                            .removes
                                            .iter()
                                            .map(|&(k, v)| (k, v.map(|v| v.clone())))
                                            .collect(),
                                        children
                                            .inserts
                                            .iter()
                                            .map(|&(k, v)| (k.map(|k| k.clone()), v))
                                            .collect(),
                                    ),
                                );
                            }

                            match diff_props_object(&id, prev_props, next_props, transaction) {
                                Some(props) => transaction.props(&id, prev_props.clone(), props),
                                None => (),
                            }
                        } else {
                            let id = view_id(parent_id, next_key.as_ref(), index);
                            self.mount_props_events(&id, next_props, transaction);
                            transaction.replace(&id, prev_view.into(), next_view.into());
                        },
                    },
                },
                None => {
                    let id = view_id(parent_id, next_view.key(), index);
                    if let Some(next_props) = next_view.props() {
                        self.mount_props_events(&id, next_props, transaction);
                    }
                    transaction.insert(parent_id, &id, index, next_view.into());
                }
            },
            None => if let Some(prev_view) = prev_view_option {
                let id = view_id(parent_id, prev_view.key(), index);
                if let Some(prev_props) = prev_view.props() {
                    self.unmount_props_events(&id, prev_props, transaction);
                }
                transaction.remove(&id, prev_view.into());
            },
        }
        */
    }

    #[inline]
    fn update_props_events(
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
                if let Some(f) = v.function() {
                    if !next_props.has(k) {
                        transaction.remove_event(id, k);
                        event_manager.remove(id, k);
                    }
                }
            }
        }
    }
}
