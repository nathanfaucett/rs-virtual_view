#[macro_use]
extern crate virtual_view;

use std::sync::mpsc::channel;

use virtual_view::{Children, Component, EventManager, Instance, Prop, Props, Renderer, Updater,
                   View};

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
                <{MyButton} onclick={ block {
                    let updater = instance.updater.clone();
                    move |_: &mut Props| App::on_add_count(&updater)
                } }>
                    {"Add"}
                </{MyButton}>
                <{MyButton} onclick={ block {
                    let updater = instance.updater.clone();
                    move |_: &mut Props| App::on_sub_count(&updater)
                } }>
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
