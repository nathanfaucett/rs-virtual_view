rs-virtual_view
=====

a virtual view in rust

```rust
extern crate messenger;
extern crate serde_json;
extern crate tokio;
#[macro_use]
extern crate virtual_view;

use messenger::unbounded_channel;
use tokio::executor::current_thread;
use virtual_view::{Children, Component, EventManager, Instance, Prop, Props, Renderer, Updater,
                   View};

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
    fn on_add_count(updater: &Updater) -> Prop {
        updater.set_state(|current| {
            let mut next = current.clone();

            next.update("count", |count| {
                if let Some(c) = count.number() {
                    *count = (c + 1.0).into();
                }
            });

            next
        });
        Prop::Null
    }
    fn on_sub_count(updater: &Updater) -> Prop {
        updater.set_state(|current| {
            let mut next = current.clone();

            next.update("count", |count| {
                if let Some(c) = count.number() {
                    *count = (c - 1.0).into();
                }
            });

            next
        });
        Prop::Null
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
                <{Button} onclick={ block {
                    let updater = instance.updater.clone();
                    move |_: &mut Props| Counter::on_add_count(&updater)
                } }>
                    {"Add"}
                </{Button}>
                <{Button} onclick={ block {
                    let updater = instance.updater.clone();
                    move |_: &mut Props| Counter::on_sub_count(&updater)
                } }>
                    {"Sub"}
                </{Button}>
            </div>
        }
    }
}

fn main() {
    let (server, client, future) = unbounded_channel();

    let event_manager = EventManager::new();
    let _renderer = Renderer::new(
        view! {
            <{Counter} count=0/>
        },
        event_manager.clone(),
        server,
    );

    let c = client.clone();
    let _ = client.on("virtual_view.transaction", move |t| {
        println!("{}", t);
        c.close();
        None
    });

    current_thread::run(move |_| {
        let _ = current_thread::spawn(future);
    });
}
```
