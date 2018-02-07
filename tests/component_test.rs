#[macro_use]
extern crate view;

use view::{Children, Component, Event, Props, Tree, Updater, View};

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
    updater.update(|props| {
        props.update("count", |count| {
            if let Some(c) = count.number() {
                *count = (c + 1.0).into();
            }
        });
    });
}
fn on_sub_count(updater: &Updater, _: &mut Event) {
    updater.update(|props| {
        props.update("count", |count| {
            if let Some(c) = count.number() {
                *count = (c - 1.0).into();
            }
        });
    });
}

impl Component for Counter {
    fn name(&self) -> &'static str {
        "Counter"
    }
    #[inline]
    fn initial_state(&self, props: &Props) -> Props {
        props! {
            "count": props.take("count").unwrap_or(0.into())
        }
    }
    fn render(&self, updater: &Updater, state: &Props, _: &Props, _: &Children) -> View {
        let count = state.take("count").unwrap_or(0.into());

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

#[test]
fn test_component_mount_unmount() {
    let (tree, receiver) = Tree::new(view! {
        <{Counter} count=0/>
    });

    tree.unmount();

    let _mount_transaction = receiver.recv().unwrap();
    let _unmount_transaction = receiver.recv().unwrap();

    println!("{:#?}", _mount_transaction);
    println!("{:#?}", _unmount_transaction);

    //assert!(false);
}
