use super::super::{view_id, Props, Transaction, View};
use super::{Node, Tree};

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
    #[inline(always)]
    fn state(&self) -> Props {
        Props::new()
    }
    #[inline]
    fn last_rendered_view(&self) -> Option<&View> {
        self.rendered_view.as_ref()
    }
    #[inline]
    fn mount(&mut self, tree: &Tree, transaction: &mut Transaction) -> View {
        let mut rendered_view = self.rendered_view();

        if let Some(props) = rendered_view.props() {
            tree.mount_props_events(&self.id, props, transaction);
        }

        if let Some(children) = rendered_view.children_mut() {
            children.iter_mut().enumerate().for_each(|(index, child)| {
                let child_id = view_id(&self.id, child.key(), index);
                *child = tree.mount_view(child_id, child.clone(), transaction);
            });
        }

        self.rendered_view = Some(rendered_view.clone());
        rendered_view
    }

    #[inline]
    fn update(&mut self, _view: View, _tree: &Tree, _transaction: &mut Transaction) -> View {
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
