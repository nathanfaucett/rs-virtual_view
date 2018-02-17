extern crate serde_json;
#[macro_use]
extern crate view;

use std::sync::mpsc::channel;

use serde_json::Map;
use view::{Children, Component, Event, EventManager, Props, Renderer, SimpleEvent, Updater, View};

struct Comp0;

impl Component for Comp0 {
    fn name(&self) -> &'static str {
        "Comp0"
    }
    fn render(&self, _: &Updater, _: &Props, _: &Props, _: &Children) -> View {
        view! {
            <p class="Comp0">{0}</p>
        }
    }
}

struct Comp1;

impl Component for Comp1 {
    fn name(&self) -> &'static str {
        "Comp1"
    }
    fn render(&self, _: &Updater, _: &Props, _: &Props, _: &Children) -> View {
        view! {
            <p class="Comp1">{1}</p>
        }
    }
}

struct TopComp;

fn switch(render: bool, updater: &Updater) {
    updater.update(|current| {
        let mut next = current.clone();
        next.insert("render", !render);
        next
    });
}

impl Component for TopComp {
    fn name(&self) -> &'static str {
        "TopComp"
    }
    fn initial_state(&self, _: &Props) -> Props {
        props! {
            "render": true,
        }
    }
    fn render(&self, updater: &Updater, state: &Props, _: &Props, _: &Children) -> View {
        let render = state.get("render").boolean().unwrap();

        let switch_updater = updater.clone();

        view! {
            <div class="TopComp">
                <button onclick={ move |_: &mut Event| switch(render, &switch_updater) }>
                    {"Switch"}
                </button>
                {
                    if render {
                        view! { <{Comp0}/> }
                    } else {
                        view! { <{Comp1}/> }
                    }
                }
            </div>
        }
    }
}

#[test]
fn test_replace_component_transaction() {
    let (sender, receiver) = channel();

    let event_manager = EventManager::new();
    let _renderer = Renderer::new(
        view! {
            <{TopComp}/>
        },
        event_manager.clone(),
        sender,
    );

    event_manager.dispatch(".0.0", &mut SimpleEvent::new("onclick", Map::new()));

    let _mount_transaction = receiver.recv().unwrap();
    let switch_transaction = receiver.recv().unwrap();

    assert!(switch_transaction.patches()[".0.1"][0].is_replace());
}
