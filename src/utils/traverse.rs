#[inline]
pub fn traverse_path<F>(start: &str, stop: &str, skip_first: bool, skip_last: bool, mut callback: F)
where
    F: FnMut(&str, bool) -> bool,
{
    let traverse_up = is_ancestor_id_of(stop, start);
    let mut id: String = start.into();
    let mut ret = true;

    loop {
        if (!skip_first || &id != start) && (!skip_last || &id != stop) {
            ret = callback(&id, traverse_up);
        }
        if ret == false || &id == stop {
            break;
        }

        id = if traverse_up {
            parent_id(&id)
        } else {
            next_descendant_id(&id, stop)
        }
    }
}

#[inline]
pub fn next_descendant_id(ancestor_id: &str, destination_id: &str) -> String {
    if ancestor_id == destination_id {
        ancestor_id.into()
    } else {
        let mut index = ancestor_id.len() + 1;
        let length = destination_id.len();

        for ch in destination_id.chars().skip(index) {
            if ch == '.' || index == length {
                break;
            } else {
                index += 1;
            }
        }

        destination_id.chars().take(index).collect()
    }
}

#[inline]
pub fn parent_id(id: &str) -> String {
    if id.len() == 0 {
        String::new()
    } else {
        match last_index_of(id, '.') {
            Some(index) => id.chars().take(index).collect(),
            None => String::new(),
        }
    }
}

#[inline]
pub fn is_boundary(id: &str, index: usize) -> bool {
    let is_point = match id.chars().nth(index) {
        Some(ch) => ch == '.',
        None => false,
    };
    is_point || index == id.len()
}

#[inline]
pub fn is_ancestor_id_of(ancestor_id: &str, descendant_id: &str) -> bool {
    descendant_id.starts_with(ancestor_id) && is_boundary(descendant_id, ancestor_id.len())
}

#[inline]
fn last_index_of(string: &str, ch: char) -> Option<usize> {
    string
        .chars()
        .rev()
        .position(|c| c == ch)
        .map(|i| string.len() - (i + 1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_last_index_of() {
        assert_eq!(last_index_of(".0.1", '.'), Some(2));
    }

    #[test]
    fn test_next_descendant_id() {
        assert_eq!(next_descendant_id("", ".0.1.2.3"), ".0".to_owned());
        assert_eq!(next_descendant_id(".", ".0.1.2.3"), ".0".to_owned());
        assert_eq!(next_descendant_id(".0", ".0.1.2.3"), ".0.1".to_owned());
        assert_eq!(
            next_descendant_id(".0.1.2.3", ".0.1.2.3"),
            ".0.1.2.3".to_owned()
        );
    }

    #[test]
    fn test_is_boundary() {
        assert_eq!(is_boundary(".0.1", 0), true);
        assert_eq!(is_boundary(".0.1", 1), false);
        assert_eq!(is_boundary(".0.1", 2), true);
        assert_eq!(is_boundary(".0.1", 3), false);
        assert_eq!(is_boundary(".0.1", 4), true);
    }

    #[test]
    fn test_parent_id() {
        assert_eq!(parent_id(".0.1"), ".0".to_owned());
        assert_eq!(parent_id(".0"), String::new());
    }

    #[test]
    fn test_is_ancestor_id_of() {
        assert_eq!(is_ancestor_id_of(".0", ".0.1"), true);
        assert_eq!(is_ancestor_id_of(".0.0", ".0.0"), true);
        assert_eq!(is_ancestor_id_of(".0,1", ".0"), false);
    }

    #[test]
    fn test_traverse_path() {
        let mut index = 0;
        let ids = [".0.1.2.3.4", ".0.1.2.3", ".0.1.2", ".0.1", ".0"];

        {
            let i = &mut index;
            traverse_path(".0.1.2.3.4", ".0", false, false, move |id, _| {
                assert_eq!(ids[*i], id);
                *i += 1;
                true
            });
        }
        {
            let i = &mut index;
            traverse_path(".0", ".0.1.2.3.4", false, false, move |id, _| {
                *i -= 1;
                assert_eq!(ids[*i], id);
                true
            });
        }
    }
}
