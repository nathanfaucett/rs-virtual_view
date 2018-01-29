use std::sync::Arc;

use super::super::{Children, Component, EventManager, Props, Transaction, View};
use super::{Node, Nodes, Updater};

pub struct ComponentNode {
    id: String,
    parent_id: String,
    state: Props,
    view: View,
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
    fn mount(
        &mut self,
        nodes: &Nodes,
        transaction: &mut Transaction,
        event_manager: &mut EventManager,
    ) {
        let rendered_view = self.rendered_view();
        println!("{:#?}", rendered_view);
    }
}

impl ComponentNode {
    #[inline]
    pub fn new(id: String, parent_id: String, view: View, component: Arc<Component>) -> Self {
        ComponentNode {
            id: id,
            parent_id: parent_id,
            state: component.initial_state(),
            view: view,
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
