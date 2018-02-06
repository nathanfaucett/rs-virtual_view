use std::fmt;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use fnv::{FnvHashMap, FnvHashSet};

use super::super::{is_ancestor_id_of, traverse_path};
use super::Event;

#[derive(Clone)]
pub struct EventManager(Arc<RwLock<FnvHashMap<String, FnvHashMap<String, Arc<Fn(&mut Event)>>>>>);

unsafe impl Send for EventManager {}
unsafe impl Sync for EventManager {}

impl EventManager {
    #[inline]
    pub fn new() -> Self {
        EventManager(Arc::new(RwLock::new(FnvHashMap::default())))
    }

    #[inline]
    pub(crate) fn read(
        &self,
    ) -> RwLockReadGuard<FnvHashMap<String, FnvHashMap<String, Arc<Fn(&mut Event)>>>> {
        self.0
            .read()
            .expect("failed to acquire EventManager read lock")
    }

    #[inline]
    pub(crate) fn write(
        &self,
    ) -> RwLockWriteGuard<FnvHashMap<String, FnvHashMap<String, Arc<Fn(&mut Event)>>>> {
        self.0
            .write()
            .expect("failed to acquire EventManager write lock")
    }

    #[inline]
    pub(crate) fn add(&self, id: &str, name: &str, func: Arc<Fn(&mut Event)>) {
        let mut write = self.write();

        if !write.contains_key(name) {
            write.insert(name.into(), FnvHashMap::default());
        }

        let funcs = write.get_mut(name).unwrap();
        funcs.insert(id.into(), func);
    }

    #[inline]
    pub(crate) fn remove(&self, id: &str, name: &str) {
        let mut write = self.write();
        let mut remove = false;

        if let Some(funcs) = write.get_mut(name) {
            funcs.remove(id);
            remove = funcs.len() == 0;
        }
        if remove {
            write.remove(name);
        }
    }

    #[inline]
    pub(crate) fn remove_all(&self, parent_id: &str) {
        let mut write = self.write();

        for (_, events) in write.iter_mut() {
            events.retain(|id, _| !is_ancestor_id_of(parent_id, id));
        }
    }

    #[inline]
    pub fn dispatch(&self, id: &str, event: &mut Event) {
        let read = self.read();

        if let Some(events) = read.get(event.name()) {
            traverse_path(id, "", false, true, |id, _| {
                if let Some(func) = events.get(id) {
                    (&*func)(event);
                }
                event.propagation()
            });
        }
    }
}

impl fmt::Debug for EventManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            self.read()
                .iter()
                .map(|(k, v)| (
                    k.clone(),
                    v.iter()
                        .map(|(i, _)| i.clone())
                        .collect::<FnvHashSet<String>>()
                ))
                .collect::<FnvHashMap<String, FnvHashSet<String>>>()
        )
    }
}
