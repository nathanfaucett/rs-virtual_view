extern crate messenger;
extern crate serde_json;
extern crate tokio;
#[macro_use]
extern crate virtual_view;

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::from_value;
use tokio::executor::current_thread;
use virtual_view::{Children, Component, EventManager, Instance, Prop, Props, Renderer,
                   Transaction, Updater, View};

struct App;

impl App {
    fn on_click(updater: &Updater) -> Prop {
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
}

impl Component for App {
    fn name(&self) -> &'static str {
        "App"
    }
    fn initial_state(&self, _: &Props) -> Props {
        props! {
            "count": 0,
        }
    }
    fn render(&self, instance: &Instance, _: &Props, _: &Children) -> View {
        let count = instance.state.take("count").unwrap().number().unwrap() % 3.0;

        let children = if count == 0.0 {
            vec![
                view! { <p key="0">{ 0 }</p> },
                view! { <p key="1">{ 1 }</p> },
                view! { <p key="2">{ 2 }</p> },
                view! { <p key="3">{ 3 }</p> },
            ]
        } else if count == 1.0 {
            vec![
                view! { <p key="2">{ 2 }</p> },
                view! { <p key="3">{ 3 }</p> },
                view! { <p key="1">{ 1 }</p> },
            ]
        } else {
            vec![
                view! { <p key="1">{ 1 }</p> },
                view! { <p key="2">{ 2 }</p> },
            ]
        };

        view! {
            <div class="App">
                <button onclick={ block {
                    let updater = instance.updater.clone();
                    move |_: &mut Props| Self::on_click(&updater)
                } }>{"Click Me!"}</button>
                <div>
                    { each children }
                </div>
            </div>
        }
    }
}

#[test]
fn test_order() {
    let (server, client, future) = messenger::unbounded_channel();

    let event_manager = EventManager::new();
    let _renderer = Renderer::new(
        view! {
            <{App}/>
        },
        event_manager.clone(),
        server,
    );

    let close_client = client.clone();
    let transactions: Arc<Mutex<Vec<Transaction>>> = Arc::new(Mutex::new(Vec::new()));
    let client_transactions = transactions.clone();
    let count = AtomicUsize::new(0);

    let _ = client.on("virtual_view.transaction", move |t| {
        if count.fetch_add(1, Ordering::SeqCst) == 2 {
            close_client.close();
        }
        client_transactions
            .lock()
            .unwrap()
            .push(from_value(t.clone()).unwrap());
        None
    });

    event_manager.dispatch(".0.0", &mut props! { "name": "onclick" });
    event_manager.dispatch(".0.0", &mut props! { "name": "onclick" });
    event_manager.dispatch(".0.0", &mut props! { "name": "onclick" });

    current_thread::run(|_| {
        let _ = current_thread::spawn(future);
    });

    let mut transactions_lock = transactions.lock().unwrap();

    let _mount_transaction = transactions_lock.remove(0);
    let update0_transaction = transactions_lock.remove(0);
    let update1_transaction = transactions_lock.remove(0);
    let update2_transaction = transactions_lock.remove(0);

    assert!(update0_transaction.patches()[".0.1"][0].is_order());
    assert!(update0_transaction.removes().contains_key(".0.1.0"));

    assert!(update1_transaction.patches()[".0.1"][0].is_order());
    assert!(update1_transaction.removes().contains_key(".0.1.3"));

    assert!(update2_transaction.patches()[".0.1"][0].is_insert());
    assert!(update2_transaction.patches()[".0.1"][1].is_insert());
    assert!(update2_transaction.patches()[".0.1"][2].is_order());
}
