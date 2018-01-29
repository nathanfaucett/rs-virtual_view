use std::sync::Arc;

use fnv::FnvHashMap;

use super::super::{is_ancestor_id_of, traverse_path};
use super::Event;

#[derive(Clone)]
pub struct EventManager(FnvHashMap<String, FnvHashMap<String, Arc<Fn(&mut Event)>>>);

unsafe impl Send for EventManager {}
unsafe impl Sync for EventManager {}

impl EventManager {
    #[inline(always)]
    pub fn new() -> Self {
        EventManager(FnvHashMap::default())
    }

    #[inline]
    pub(crate) fn add(&mut self, id: &str, name: &str, func: Arc<Fn(&mut Event)>) {
        if !self.0.contains_key(name) {
            self.0.insert(name.into(), FnvHashMap::default());
        }

        let funcs = self.0.get_mut(name).unwrap();
        funcs.insert(id.into(), func);
    }

    #[inline]
    pub(crate) fn remove(&mut self, id: &str, name: &str) {
        let mut remove = false;

        if let Some(funcs) = self.0.get_mut(name) {
            funcs.remove(id);
            remove = funcs.len() == 0;
        }
        if remove {
            self.0.remove(name);
        }
    }

    #[inline]
    pub(crate) fn remove_all(&mut self, parent_id: &str) {
        for (_, events) in &mut self.0 {
            events.retain(|id, _| !is_ancestor_id_of(parent_id, id));
        }
    }

    #[inline]
    pub fn dispatch(&self, id: &str, event: &mut Event) {
        if let Some(events) = self.0.get(event.name()) {
            traverse_path(id, "", false, true, |id, _| {
                if let Some(func) = events.get(id) {
                    (&*func)(event);
                }
                event.propagation()
            });
        }
    }
}
