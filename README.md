rs-virtual_view
=====

a virtual view in rust

```rust
extern crate serde_json;
#[macro_use]
extern crate virtual_view;

use std::sync::mpsc::channel;

use virtual_view::{Children, Component, EventManager, Instance, Props, Renderer,
           Updater, View};
use serde_json::Map;

struct MyButton;

impl Component for MyButton {
    fn render(&self, _: &Instance, props: &Props, children: &Children) -> View {
        view! {
            <button class="Button" ... { props }>{ each children }</button>
        }
    }
}

struct App;

impl App {
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
}

impl Component for App {
    fn name(&self) -> &'static str {
        "App"
    }
    fn initial_state(&self, props: &Props) -> Props {
        props! {
            "count": props.take("count").unwrap_or(0.into())
        }
    }
    fn render(&self, instance: &Instance, _: &Props, _: &Children) -> View {
        view! {
            <div class="App">
                <p>{format!("Count {}", instance.state.get("count"))}</p>
                <{MyButton} onclick={ instance.wrap(App::on_add_count) }>
                    {"Add"}
                </{MyButton}>
                <{MyButton} onclick={ instance.wrap(App::on_sub_count) }>
                    {"Sub"}
                </{MyButton}>
            </div>
        }
    }
}

fn main() {
    let (sender, receiver) = channel();

    let event_manager = EventManager::new();
    let renderer = Renderer::new(
        view! {
            <{App} count=0/>
        },
        event_manager.clone(),
        sender,
    );

    event_manager.dispatch(".0.1", &mut props! { "name": "onclick" });

    let mount_transaction = receiver.recv().unwrap();
    println!("{:#?}", mount_transaction);

    renderer.unmount();
}
```
