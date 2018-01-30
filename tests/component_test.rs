#[macro_use]
extern crate view;

use std::sync::Arc;
use view::{Children, Component, Event, EventManager, Prop, Props, Tree, Updater, View};

struct Button;

impl Component for Button {
    fn name(&self) -> &'static str {
        "Button"
    }
    fn render(&self, updater: Updater, state: &Props, props: &Props, children: &Children) -> View {
        let mut props = props.clone();
        props.insert("class", "Button");
        View::new("button".into(), props, vec![])
    }
}

struct Counter;

fn on_add_count(_: Updater, _: &mut Event) {}
fn on_sub_count(_: Updater, _: &mut Event) {}

impl Component for Counter {
    fn name(&self) -> &'static str {
        "Counter"
    }
    #[inline]
    fn initial_state(&self, props: &Props) -> Props {
        let mut state = Props::default();
        state.insert("count", props.take("count").unwrap_or(0.into()));
        state
    }
    fn render(&self, updater: Updater, state: &Props, props: &Props, children: &Children) -> View {
        let mut add = View::new_component(Button);
        add.props_mut().unwrap().insert(
            "click",
            Prop::Function(Arc::new(move |e| {
                on_add_count(updater, e);
            })),
        );
        add.children_mut().unwrap().push("Add".into());

        let mut sub = View::new_component(Button);
        sub.props_mut().unwrap().insert(
            "click",
            Prop::Function(Arc::new(move |e| {
                on_sub_count(updater, e);
            })),
        );
        sub.children_mut().unwrap().push("Sub".into());

        let count = state.get("count").map(|c| c.clone()).unwrap_or(0.into());
        let text = View::new(
            "p".into(),
            Props::default(),
            vec![format!("Count {}", count).into()],
        );

        View::new("div".into(), Props::default(), vec![text, add, sub])
    }
}

#[test]
fn test_component() {
    let mut tree = Tree::new();
    let mut event_manager = EventManager::new();

    let mut view = View::new_component(Counter);
    view.props_mut().unwrap().insert("count", 0);

    tree.render(view, &mut event_manager);
    println!(
        "{:?}",
        tree.nodes
            .lock()
            .iter()
            .map(|(id, _)| id)
            .collect::<Vec<_>>()
    );
    assert!(false);
}
