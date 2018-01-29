#[inline]
pub fn view_id(parent_id: &str, child_key: Option<&String>, index: usize) -> String {
    let child_view_id = child_view_id(child_key, index);

    let mut string = String::with_capacity(parent_id.len() + child_view_id.len() + 1);

    string.push_str(parent_id);
    string.push('.');
    string.push_str(&child_view_id);

    string
}

#[inline]
pub fn child_view_id(child_key: Option<&String>, index: usize) -> String {
    match child_key {
        None => index.to_string(),
        Some(key) => key.clone(),
    }
}
