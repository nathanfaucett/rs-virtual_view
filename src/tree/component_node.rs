use std::sync::Arc;

use super::super::{view_id, Children, Component, EventManager, Props, Transaction, View};
use super::{Node, Nodes, Tree, Updater};

pub struct ComponentNode {
    id: String,
    parent_id: String,
    state: Props,
    view: View,
    rendered_view: Option<View>,
    updater: Updater,
    component: Arc<Component>,
}

impl Node for ComponentNode {
    #[inline(always)]
    fn id(&self) -> &String {
        &self.id
    }
    #[inline(always)]
    fn parent_id(&self) -> &String {
        &self.parent_id
    }
    #[inline]
    fn last_rendered_view(&self) -> Option<&View> {
        self.rendered_view.as_ref()
    }
    #[inline]
    fn mount(
        &mut self,
        nodes: &Nodes,
        event_manager: &EventManager,
        transaction: &mut Transaction,
    ) -> View {
        let mut rendered_view = self.rendered_view();

        if let Some(props) = rendered_view.props() {
            Tree::mount_props_events(event_manager, &self.id, props, transaction);
        }

        if let Some(children) = rendered_view.children_mut() {
            children.iter_mut().enumerate().for_each(|(index, child)| {
                if child.is_data() {
                    let child_id = view_id(&self.id, child.key(), index);

                    *child = Tree::mount_view(
                        nodes,
                        event_manager,
                        child_id,
                        child.clone(),
                        transaction,
                    );
                }
            });
        }

        self.rendered_view = Some(rendered_view.clone());
        rendered_view
    }

    #[inline]
    fn update(
        &mut self,
        _view: View,
        _nodes: &Nodes,
        _event_manager: &EventManager,
        _transaction: &mut Transaction,
    ) -> View {
        let rendered_view = self.rendered_view();
        rendered_view
    }
}

impl ComponentNode {
    #[inline]
    pub fn new(id: String, parent_id: String, view: View, component: Arc<Component>) -> Self {
        ComponentNode {
            id: id,
            parent_id: parent_id,
            state: component.initial_state(view.props().unwrap()),
            view: view,
            rendered_view: None,
            updater: Updater,
            component: component,
        }
    }

    #[inline]
    fn rendered_view(&self) -> View {
        let empty_props = Props::default();
        let empty_children = Children::new();

        let state = &self.state;
        let props = self.view.props().unwrap_or(&empty_props);
        let children = self.view.children().unwrap_or(&empty_children);

        self.component
            .render(self.updater.clone(), state, props, children)
    }
}
