extern crate serde_json;
#[macro_use]
extern crate view;

use std::sync::mpsc::channel;

use serde_json::Map;
use view::{Children, Component, Event, EventManager, Instance, Props, Renderer, SimpleEvent,
           Updater, View};

struct Button;

impl Component for Button {
    fn name(&self) -> &'static str {
        "Button"
    }
    fn render(&self, _: &Instance, props: &Props, children: &Children) -> View {
        view! {
            <button class="Button" ... { props }>{ each children }</button>
        }
    }
}

struct Counter;

fn on_add_count(updater: &Updater, _: &mut Event) {
    updater.set_state(|current| {
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
    updater.set_state(|current| {
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
    fn initial_state(&self, props: &Props) -> Props {
        props! {
            "count": props.take("count").unwrap_or(0.into())
        }
    }
    fn render(&self, instance: &Instance, _: &Props, _: &Children) -> View {
        let count = instance.state.get("count");

        let add_updater = instance.updater.clone();
        let sub_updater = instance.updater.clone();

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

#[test]
fn test_component_transaction() {
    let (sender, receiver) = channel();

    let event_manager = EventManager::new();
    let renderer = Renderer::new(
        view! {
            <{Counter} count=0/>
        },
        event_manager.clone(),
        sender,
    );

    event_manager.dispatch(".0.1", &mut SimpleEvent::new("onclick", Map::new()));
    event_manager.dispatch(".0.2", &mut SimpleEvent::new("onclick", Map::new()));
    event_manager.dispatch(".0.1", &mut SimpleEvent::new("onclick", Map::new()));

    renderer.unmount();

    let mount_transaction = receiver.recv().unwrap();
    let add0_update_transaction = receiver.recv().unwrap();
    let sub_update_transaction = receiver.recv().unwrap();
    let add1_update_transaction = receiver.recv().unwrap();
    let unmount_transaction = receiver.recv().unwrap();

    assert!(&mount_transaction.patches()[".0"][0].is_mount());
    assert!(&add0_update_transaction.patches()[".0.0.0"][0].is_replace());
    assert!(&sub_update_transaction.patches()[".0.0.0"][0].is_replace());
    assert!(&add1_update_transaction.patches()[".0.0.0"][0].is_replace());
    assert!(&unmount_transaction.removes().contains_key(".0"));
}
