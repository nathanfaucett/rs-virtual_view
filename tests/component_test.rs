extern crate serde_json;
#[macro_use]
extern crate view;

use serde_json::{Map, Value};
use view::{Children, Component, Event, Props, Tree, Updater, View};

struct Button;

impl Component for Button {
    fn name(&self) -> &'static str {
        "Button"
    }
    fn render(&self, _: &Updater, _: &Props, props: &Props, children: &Children) -> View {
        view! {
            <button class="Button" ... { props }>{ each children }</button>
        }
    }
}

struct Counter;

fn on_add_count(updater: &Updater, _: &mut Event) {
    updater.update(|current| {
        let mut next = current.clone();

        next.update("count", |count| {
            if let Some(c) = count.number() {
                *count = (c + 1.0).into();
            }
        });

        next
    });
}
fn on_sub_count(updater: &Updater, _: &mut Event) {
    updater.update(|current| {
        let mut next = current.clone();

        next.update("count", |count| {
            if let Some(c) = count.number() {
                *count = (c - 1.0).into();
            }
        });

        next
    });
}

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
    fn render(&self, updater: &Updater, state: &Props, _: &Props, _: &Children) -> View {
        let count = state.get("count");

        let add_updater = updater.clone();
        let sub_updater = updater.clone();

        view! {
            <div class="Counter">
                <p>{format!("Count {}", count)}</p>
                <{Button} onclick={ move |e: &mut Event| on_add_count(&add_updater, e) }>
                    {"Add"}
                </{Button}>
                <{Button} onclick={ move |e: &mut Event| on_sub_count(&sub_updater, e) }>
                    {"Sub"}
                </{Button}>
            </div>
        }
    }
}

pub struct TestEvent {
    name: String,
    data: Map<String, Value>,
    propagation: bool,
}

impl TestEvent {
    fn new<T>(name: T) -> Self
    where
        T: ToString,
    {
        TestEvent {
            name: name.to_string(),
            data: Map::new(),
            propagation: true,
        }
    }
}

impl Event for TestEvent {
    fn name(&self) -> &String {
        &self.name
    }
    fn data(&self) -> &Map<String, Value> {
        &self.data
    }
    fn propagation(&self) -> bool {
        self.propagation
    }
    fn stop_propagation(&mut self) {
        self.propagation = false;
    }
}

#[test]
fn test_component_mount_unmount() {
    let (tree, receiver) = Tree::new(view! {
        <{Counter} count=0/>
    });
    let event_manager = tree.event_manager();

    event_manager.dispatch(".0.1", &mut TestEvent::new("onclick"));
    event_manager.dispatch(".0.2", &mut TestEvent::new("onclick"));

    tree.unmount();

    let _mount_transaction = receiver.recv().unwrap();
    let _add_update_transaction = receiver.recv().unwrap();
    let _sub_update_transaction = receiver.recv().unwrap();
    let _unmount_transaction = receiver.recv().unwrap();

    println!("mount {:#?}", _mount_transaction);
    println!("add {:#?}", _add_update_transaction);
    println!("sub {:#?}", _sub_update_transaction);
    println!("unmount {:#?}", _unmount_transaction);

    assert!(false);
}
