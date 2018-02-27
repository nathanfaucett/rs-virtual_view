extern crate messenger;
extern crate serde_json;
extern crate tokio;
#[macro_use]
extern crate virtual_view;

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::executor::current_thread;
use virtual_view::{Children, Component, EventManager, Instance, Props, Renderer, View};

struct Comp0;

impl Component for Comp0 {
    fn name(&self) -> &'static str {
        "Comp0"
    }
    fn inherit_context(&self, mut context: Props, parent_context: &Props) -> Props {
        context.extend(parent_context);
        context
    }
    fn render(&self, instance: &Instance, _: &Props, _: &Children) -> View {
        let color = instance.context.take("color").unwrap();

        view! {
            <p class="Comp0" style={{ "color": color }}>{0}</p>
        }
    }
}

struct Comp1;

impl Component for Comp1 {
    fn name(&self) -> &'static str {
        "Comp1"
    }
    fn inherit_context(&self, mut context: Props, parent_context: &Props) -> Props {
        context.extend(parent_context);
        context
    }
    fn render(&self, instance: &Instance, _: &Props, _: &Children) -> View {
        let color = instance.context.take("color").unwrap();

        view! {
            <p class="Comp1" style={{ "color": color }}>{1}</p>
        }
    }
}

struct TopComp;

impl Component for TopComp {
    fn name(&self) -> &'static str {
        "TopComp"
    }
    fn initial_state(&self, _: &Props) -> Props {
        props! {
            "render": true,
        }
    }
    fn context(&self, _: &Props) -> Props {
        props! {
            "color": "#F00",
        }
    }
    fn will_mount(&self, instance: &Instance) {
        instance.set_state(|current| {
            let render = current.get("render").boolean().unwrap();
            let mut next = current.clone();
            next.insert("render", !render);
            next
        });
    }
    fn render(&self, instance: &Instance, _: &Props, _: &Children) -> View {
        view! {
            <div class="TopComp">
                {
                    if instance.state.get("render").boolean().unwrap() {
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
    let (server, client, future) = messenger::unbounded_channel();

    let event_manager = EventManager::new();
    let _renderer = Renderer::new(
        view! {
            <{TopComp}/>
        },
        event_manager.clone(),
        server,
    );

    let close_client = client.clone();
    let transactions = Arc::new(Mutex::new(Vec::new()));
    let client_transactions = transactions.clone();
    let count = AtomicUsize::new(0);

    let _ = client.on("virtual_view.transaction", move |t| {
        if count.fetch_add(1, Ordering::SeqCst) == 1 {
            close_client.close();
        }
        client_transactions.lock().unwrap().push(t.clone());
        None
    });

    current_thread::run(|_| {
        let _ = current_thread::spawn(future);
    });

    let mut transactions_lock = transactions.lock().unwrap();

    let _mount_transaction = transactions_lock.remove(0);
    let switch_transaction = transactions_lock.remove(0);

    assert!(switch_transaction.patches()[".0.0"][0].is_replace());
}
