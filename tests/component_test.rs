#[macro_use]
extern crate view;

use view::{Children, Component, Event, Props, Tree, Updater, View};

struct Button;

impl Component for Button {
    fn name(&self) -> &'static str {
        "Button"
    }
    fn render(&self, _: Updater, _: &Props, props: &Props, children: &Children) -> View {
        view! {
            <button class="Button" ... { props }>{ each children }</button>
        }
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
        props! {
            "count": props.take("count").unwrap_or(0.into())
        }
    }
    fn render(&self, updater: Updater, state: &Props, _: &Props, _: &Children) -> View {
        let count = state.take("count").unwrap_or(0.into());

        view! {
            <div class="Counter">
                <p>{format!("Count {}", count)}</p>
                <{Button} onclick={ move |e: &mut Event| on_add_count(updater, e) }>
                    {"Add"}
                </{Button}>
                <{Button} onclick={ move |e: &mut Event| on_sub_count(updater, e) }>
                    {"Sub"}
                </{Button}>
            </div>
        }
    }
}

#[test]
fn test_component() {
    let tree = Tree::new();

    let mut view = View::new_component(Counter);
    view.props_mut().unwrap().insert("count", 0);

    let _mount_transaction = tree.mount(view);
    let _unmount_transaction = tree.unmount();

    assert!(false);
}
