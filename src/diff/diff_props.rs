use super::super::{Prop, Props};

#[inline]
pub fn diff_props(prev: &Prop, next: &Prop) -> Prop {
    match prev {
        &Prop::Object(ref prev_object) => match next {
            &Prop::Object(ref next_object) => match diff_props_object(prev_object, next_object) {
                Some(object) => Prop::Object(object),
                None => Prop::Null,
            },
            &Prop::Function(_) => Prop::Null,
            _ => next.clone(),
        },
        _ => next.clone(),
    }
}

#[inline]
pub fn diff_props_object(prev_object: &Props, next_object: &Props) -> Option<Props> {
    let mut result = Props::new();

    for (key, prev_value) in prev_object {
        match next_object.get(key) {
            &Prop::Null => {
                result.insert(key.clone(), Prop::Null);
            }
            &Prop::Function(_) => (),
            next_value => if prev_value != next_value {
                result.insert(key.clone(), diff_props(prev_value, next_value));
            },
        }
    }

    for (key, next_value) in next_object {
        match prev_object.get(key) {
            &Prop::Null => {
                result.insert(key.clone(), next_value.clone());
            }
            _ => (),
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}
