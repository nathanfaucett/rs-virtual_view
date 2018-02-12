use fnv::FnvHashMap;

use super::super::View;

#[inline]
pub fn diff_children<'a>(prev_children: &'a [View], next_children: &'a [View]) -> DiffChildren<'a> {
    let prev_children_len = prev_children.len();
    let next_children_len = next_children.len();
    let max_children = if prev_children_len > next_children_len {
        prev_children_len
    } else {
        next_children_len
    };
    let next_indices = KeyIndices::new(next_children);

    if next_indices.free.len() != prev_children_len {
        let prev_indices = KeyIndices::new(prev_children);

        if prev_indices.free.len() != prev_children_len {
            let mut children = DiffChildren::new();

            let mut free_index = 0;
            let free_count = next_indices.free.len();
            let mut deleted_items = 0;

            for prev_item in prev_children {
                if let Some(prev_item_key) = prev_item.key() {
                    if let Some(item_index) = next_indices.keys.get(prev_item_key) {
                        children.children.push(next_children.get(*item_index));
                    } else {
                        deleted_items += 1;
                        children.children.push(None);
                    }
                } else {
                    if free_index < free_count {
                        let item_index = next_indices.free[free_index];
                        free_index += 1;
                        children.children.push(next_children.get(item_index));
                    } else {
                        deleted_items += 1;
                        children.children.push(None);
                    }
                }
            }

            let last_free_index = if free_index >= next_indices.free.len() {
                next_children_len
            } else {
                next_indices.free[free_index]
            };

            let mut j = 0;
            for new_item in next_children {
                if let Some(new_item_key) = new_item.key() {
                    if !prev_indices.keys.contains_key(new_item_key) {
                        children.children.push(Some(new_item));
                    }
                } else if j >= last_free_index {
                    children.children.push(Some(new_item));
                }
                j += 1;
            }

            let mut simulate = children.children.clone();
            let mut simulate_index = 0;

            let mut k = 0;
            while k < next_children_len {
                let wanted_item = &next_children[k];
                let mut simulate_item = simulate[simulate_index];

                while simulate_item == None && simulate.len() != 0 {
                    simulate.remove(simulate_index);
                    children.removes.push((simulate_index, None));
                    simulate_item = simulate[simulate_index];
                }

                if simulate_item == None || simulate_item.unwrap().key() != wanted_item.key() {
                    if let Some(wanted_item_key) = wanted_item.key() {
                        if simulate_item.is_some() && simulate_item.unwrap().key().is_some() {
                            let simulate_item_unwraped = simulate_item.unwrap();
                            let simulate_item_key = simulate_item_unwraped.key().unwrap();

                            if next_indices.keys[simulate_item_key] != k + 1 {
                                simulate.remove(simulate_index);
                                children
                                    .removes
                                    .push((simulate_index, Some(simulate_item_key)));
                                simulate_item = simulate[simulate_index];

                                if simulate_item.is_none() || simulate_item_key != wanted_item_key {
                                    children.inserts.push((Some(wanted_item_key), k));
                                } else {
                                    simulate_index += 1;
                                }
                            } else {
                                children.inserts.push((Some(wanted_item_key), k));
                            }
                        } else {
                            children.inserts.push((Some(wanted_item_key), k));
                        }
                        k += 1
                    } else if let Some(simulate_item) = simulate_item {
                        let simulate_item_key = simulate_item.key();

                        if simulate_item_key.is_some() {
                            simulate.remove(simulate_index);
                            children.removes.push((simulate_index, simulate_item_key));
                        }
                    }
                } else {
                    simulate_index += 1;
                    k += 1;
                }
            }

            while simulate_index < simulate.len() {
                let simulate_item = simulate[simulate_index];
                simulate.remove(simulate_index);
                children.removes.push((
                    simulate_index,
                    if let Some(simulate_item) = simulate_item {
                        simulate_item.key()
                    } else {
                        None
                    },
                ));
            }

            if children.removes.len() == deleted_items && children.inserts.len() == 0 {
                children.clear();
                children.pad(max_children)
            } else {
                children.pad(max_children)
            }
        } else {
            DiffChildren::from(&*next_children).pad(max_children)
        }
    } else {
        DiffChildren::from(&*next_children).pad(max_children)
    }
}

pub struct DiffChildren<'a> {
    children: Vec<Option<&'a View>>,
    removes: Vec<(usize, Option<&'a String>)>,
    inserts: Vec<(Option<&'a String>, usize)>,
}

impl<'a> From<&'a [View]> for DiffChildren<'a> {
    #[inline(always)]
    fn from(children: &'a [View]) -> Self {
        DiffChildren {
            children: children.iter().map(|v| Some(v)).collect(),
            removes: Vec::new(),
            inserts: Vec::new(),
        }
    }
}

impl<'a> DiffChildren<'a> {
    #[inline(always)]
    fn new() -> Self {
        DiffChildren {
            children: Vec::new(),
            removes: Vec::new(),
            inserts: Vec::new(),
        }
    }
    #[inline]
    fn clear(&mut self) {
        self.removes.clear();
        self.inserts.clear();
    }
    #[inline]
    fn pad(mut self, len: usize) -> Self {
        if self.children.len() < len {
            for _ in 0..(len - self.children.len()) {
                self.children.push(None);
            }
        }
        self
    }
}

struct KeyIndices<'a> {
    keys: FnvHashMap<&'a String, usize>,
    free: Vec<usize>,
}

impl<'a> KeyIndices<'a> {
    #[inline]
    fn new(children: &'a [View]) -> Self {
        let mut keys = FnvHashMap::default();
        let mut free = Vec::new();

        for (i, child) in children.iter().enumerate() {
            if let Some(key) = child.key() {
                keys.insert(key, i);
            } else {
                free.push(i);
            }
        }

        KeyIndices {
            keys: keys,
            free: free,
        }
    }
}
