use super::super::{view_id, Order, Props, Transaction, Tree, View};
use super::{diff_children, diff_props_map};

#[inline]
pub fn diff_view(
    tree: &Tree,
    parent_id: &str,
    index: usize,
    prev_view_option: Option<&View>,
    next_view_option: Option<&View>,
    transaction: &mut Transaction,
) -> Option<View> {
    match next_view_option {
        Some(next_view) => match prev_view_option {
            Some(prev_view) => match prev_view {
                &View::Text(ref prev_text) => match next_view {
                    &View::Text(ref next_text) => if prev_text != next_text {
                        let id = view_id(parent_id, prev_view.key(), index);
                        transaction.replace(&id, prev_view.into(), next_view.into());
                    },
                    &View::Data {
                        props: ref next_props,
                        ..
                    } => {
                        let id = view_id(parent_id, prev_view.key(), index);
                        add_events(&id, next_props, tree.event_manager(), transaction);
                        transaction.replace(&id, prev_view.into(), next_view.into());
                    }
                },
                &View::Data {
                    key: ref prev_key,
                    props: ref prev_props,
                    children: ref prev_children,
                    ..
                } => match next_view {
                    &View::Text(_) => {
                        let id = view_id(parent_id, prev_view.key(), index);
                        remove_events(&id, prev_events, tree.event_manager(), transaction);
                        transaction.replace(&id, prev_view.into(), next_view.into());
                    }
                    &View::Data {
                        key: ref next_key,
                        props: ref next_props,
                        events: ref next_events,
                        children: ref next_children,
                        ..
                    } => if prev_key == next_key {
                        let children = diff_children(prev_children, next_children);
                        let id = view_id(parent_id, next_key.as_ref(), index);

                        for i in 0..children.children.len() {
                            diff_view(
                                &id,
                                i,
                                prev_children.get(i),
                                children.children[i],
                                tree.event_manager(),
                                transaction,
                            );
                        }

                        if children.removes.len() != 0 || children.inserts.len() != 0 {
                            transaction.order(
                                &id,
                                Order::new(
                                    children
                                        .removes
                                        .iter()
                                        .map(|&(k, v)| (k, v.map(|v| v.clone())))
                                        .collect(),
                                    children
                                        .inserts
                                        .iter()
                                        .map(|&(k, v)| (k.map(|k| k.clone()), v))
                                        .collect(),
                                ),
                            );
                        }

                        diff_events(
                            &id,
                            prev_events,
                            next_events,
                            tree.event_manager(),
                            transaction,
                        );

                        match diff_props_map(prev_props, next_props) {
                            Some(props) => transaction.props(&id, prev_props.clone(), props),
                            None => (),
                        }
                    } else {
                        let id = view_id(parent_id, next_key.as_ref(), index);
                        add_events(&id, next_events, tree.event_manager(), transaction);
                        transaction.replace(&id, prev_view.into(), next_view.into());
                    },
                },
            },
            None => {
                let id = view_id(parent_id, next_view.key(), index);
                if let Some(next_events) = next_view.events() {
                    add_events(&id, next_events, tree.event_manager(), transaction);
                }
                transaction.insert(parent_id, &id, index, next_view.into());
            }
        },
        None => if let Some(prev_view) = prev_view_option {
            let id = view_id(parent_id, prev_view.key(), index);
            if let Some(prev_events) = prev_view.events() {
                remove_events(&id, prev_events, event_manager, transaction);
            }
            transaction.remove(&id, prev_view.into());
        },
    }
}

#[inline]
fn remove_events(
    id: &str,
    events: &Props,
    event_manager: &EventManager,
    transaction: &mut Transaction,
) {
    events.for_each(|(name, _)| {
        transaction.remove_event(id, name);
        event_manager.remove(id, name);
    });
}

#[inline]
fn add_events(
    id: &str,
    events: &Props,
    event_manager: &EventManager,
    transaction: &mut Transaction,
) {
    events.for_each(|(name, func)| {
        transaction.add_event(id, name);
        event_manager.add(id, name, func.clone());
    });
}

#[inline]
fn diff_events(
    id: &str,
    prev_events: &Props,
    next_events: &Props,
    event_manager: &EventManager,
    transaction: &mut Transaction,
) {
    prev_events.for_each(|(name, _)| {
        if !next_events.contains(name) {
            transaction.remove_event(id, name);
            event_manager.remove(id, name);
        }
    });
    next_events.for_each(|(name, func)| {
        if !prev_events.contains(name) {
            transaction.add_event(id, name);
        }
        event_manager.add(id, name, func.clone());
    });
}
