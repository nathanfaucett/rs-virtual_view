use super::super::{view_id, EventManager, Transaction, View};
use super::{Node, Nodes, Tree};

pub struct ViewNode {
    id: String,
    parent_id: String,
    view: View,
    rendered_view: Option<View>,
}

impl Node for ViewNode {
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
    ) -> View {
        let mut rendered_view = self.rendered_view();

        if let Some(props) = rendered_view.props() {
            Tree::mount_props_events(&self.id, props, transaction, event_manager);
        }

        if let Some(children) = rendered_view.children_mut() {
            children.iter_mut().enumerate().for_each(|(index, child)| {
                let child_id = view_id(&self.id, child.key(), index);
                *child =
                    Tree::mount_view(nodes, child_id, child.clone(), transaction, event_manager);
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
        _transaction: &mut Transaction,
        _event_manager: &mut EventManager,
    ) -> View {
        let rendered_view = self.rendered_view();
        rendered_view
    }
}

impl ViewNode {
    #[inline]
    pub fn new(id: String, parent_id: String, view: View) -> Self {
        ViewNode {
            id: id,
            parent_id: parent_id,
            view: view,
            rendered_view: None,
        }
    }
    #[inline]
    fn rendered_view(&self) -> View {
        self.view.clone()
    }
}
