use super::super::{Prop, Props};

// TODO: add/remove events to event manager
#[inline]
pub fn diff_props(prev: &Prop, next: &Prop) -> Prop {
    match prev {
        &Prop::Map(ref prev_map) => match next {
            &Prop::Map(ref next_map) => match diff_props_map(prev_map, next_map) {
                Some(map) => Prop::Map(map),
                None => Prop::Null,
            },
            _ => next.clone(),
        },
        _ => next.clone(),
    }
}

#[inline]
pub fn diff_props_map(prev_map: &Props, next_map: &Props) -> Option<Props> {
    let mut result = Props::default();

    for (key, prev_value) in prev_map {
        match next_map.get(key) {
            Some(next_value) => if prev_value != next_value {
                result.insert(key.clone(), diff_props(prev_value, next_value));
            },
            None => {
                result.insert(key.clone(), Prop::Null);
            }
        }
    }

    for (key, next_value) in next_map {
        match prev_map.get(key) {
            Some(_) => (),
            None => {
                result.insert(key.clone(), next_value.clone());
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}
