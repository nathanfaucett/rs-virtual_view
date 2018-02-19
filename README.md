rs-view
=====

a virtual view in rust

```rust
extern crate serde_json;
#[macro_use]
extern crate view;

use std::sync::mpsc::channel;

use serde_json::Map;
use view::{Children, Component, Event, EventManager, Props, Renderer, SimpleEvent, Updater, View};

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

    let mount_transaction = receiver.recv().unwrap();
    println!("{:?}", mount_transaction);
}
```
