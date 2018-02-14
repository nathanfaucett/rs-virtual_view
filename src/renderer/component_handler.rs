use std::sync::Arc;

use super::super::{Children, Component, Props, Transaction, Updater, View};
use super::{Handler, Node, Renderer};

pub struct ComponentHandler {
    renderer: Renderer,
    node: Node,
    updater: Updater,
    view: View,
    state: Props,
    component: Arc<Component>,
}

impl ComponentHandler {
    #[inline]
    pub fn new(renderer: &Renderer, view: View, component: Arc<Component>) -> Self {
        let state = component.initial_state(view.props().unwrap());
        let updater = Updater::new(renderer.clone());
        let rendered_view = Self::render_component_view(&view, &state, &component, &updater);

        ComponentHandler {
            renderer: renderer.clone(),
            node: Node::new(renderer, rendered_view),
            updater: updater,
            view: view,
            state: state,
            component: component,
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
    fn render_view(&self) -> View {
        Self::render_component_view(&self.view, &self.state, &self.component, &self.updater)
    }

    #[inline]
    fn render_view_with_state(&self, state: &Props) -> View {
        Self::render_component_view(&self.view, state, &self.component, &self.updater)
    }
}

impl Handler for ComponentHandler {
    #[inline]
    fn mount(&mut self, transaction: &mut Transaction) -> View {
        let mut view = self.node.mount(transaction);

        self.component.will_mount(&self.updater);

        match &mut view {
            &mut View::Data {
                ref mut children,
                ref props,
                ..
            } => for (index, child) in children.iter_mut().enumerate() {
                let node = Node::new(&self.renderer, child.clone());
                *child = node.mount(transaction);
            },
            _ => (),
        }

        view
    }

    #[inline]
    fn receive(&mut self, next_view: View) {}

    #[inline]
    fn update(&mut self, prev_view: View, next_view: View) {}
}
