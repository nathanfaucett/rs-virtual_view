extern crate serde_json;
#[macro_use]
extern crate virtual_view;

use std::sync::mpsc::channel;

use serde_json::Map;
use virtual_view::{Children, Component, Event, EventManager, Instance, Props, Renderer,
                   SimpleEvent, Updater, View};

struct Button;

impl Component for Button {
    fn name(&self) -> &'static str {
        "Button"
    }
    fn render(&self, _: &Instance, props: &Props, children: &Children) -> View {
        view! {
            <button class="Button" ...{ props }>{ each children }</button>
        }
    }
}

struct Counter;

impl Counter {
    fn on_add_count(updater: &Updater) {
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
    fn on_sub_count(updater: &Updater) {
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
        view! {
            <div class="Counter">
                <p>{format!("Count {}", instance.state.get("count"))}</p>
                <{Button} onclick={ event {
                    let updater = instance.updater.clone();
                    move |_: &mut Event| Counter::on_add_count(&updater)
                } }>
                    {"Add"}
                </{Button}>
                <{Button} onclick={ event {
                    let updater = instance.updater.clone();
                    move |_: &mut Event| Counter::on_sub_count(&updater)
                } }>
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
