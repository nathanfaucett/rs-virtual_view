rs-view
=====

a virtual view in rust

```rust
#[macro_use]
extern crate view;
extern crate serde_json;

use std::sync::mpsc::channel;

use view::{Children, Component, Event, EventManager, Props, SimpleEvent, Renderer, Instance, SimpleEvent, Updater, View};
use serde_json::Map;

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

impl Counter {
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
                <{Button} onclick={ instance.wrap(Counter::on_add_count) }>
                    {"Add"}
                </{Button}>
                <{Button} onclick={ instance.wrap(Counter::on_sub_count) }>
                    {"Sub"}
                </{Button}>
            </div>
        }
    }
}

fn main() {
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

    let mount_transaction = receiver.recv().unwrap();
    println!("{:?}", mount_transaction);
}
```
