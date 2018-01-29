use super::super::Prop;

#[inline]
pub fn prop_to_string_take(prop: Prop) -> String {
    if let Prop::String(string) = prop {
        string
    } else {
        prop.to_string()
    }
}

#[inline]
pub fn prop_to_string(prop: &Prop) -> String {
    if let &Prop::String(ref string) = prop {
        string.clone()
    } else {
        prop.to_string()
    }
}
