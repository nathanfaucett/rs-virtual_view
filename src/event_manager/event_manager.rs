use std::fmt;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use fnv::{FnvHashMap, FnvHashSet};

use super::super::traverse_path;
use super::Event;

#[derive(Clone)]
pub struct EventManager(Arc<RwLock<EventManagerInner>>);

unsafe impl Send for EventManager {}
unsafe impl Sync for EventManager {}

impl EventManager {
    #[inline]
    pub fn new() -> Self {
        EventManager(Arc::new(RwLock::new(EventManagerInner::new())))
    }

    #[inline]
    pub(crate) fn read(&self) -> RwLockReadGuard<EventManagerInner> {
        self.0
            .read()
            .expect("failed to acquire EventManager read lock")
    }

    #[inline]
    pub(crate) fn write(&self) -> RwLockWriteGuard<EventManagerInner> {
        self.0
            .write()
            .expect("failed to acquire EventManager write lock")
    }

    #[inline]
    pub fn dispatch(&self, id: &str, event: &mut Event) {
        self.read().dispatch(id, event)
    }
}

impl fmt::Debug for EventManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            self.read()
                .0
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

pub(crate) struct EventManagerInner(FnvHashMap<String, FnvHashMap<String, Arc<Fn(&mut Event)>>>);

impl EventManagerInner {
    #[inline]
    fn new() -> Self {
        EventManagerInner(FnvHashMap::default())
    }
    #[inline]
    pub(crate) fn add(&mut self, id: &str, name: &str, func: Arc<Fn(&mut Event)>) {
        self.0
            .entry(name.into())
            .or_insert_with(FnvHashMap::default)
            .insert(id.into(), func);
    }

    #[inline]
    pub(crate) fn remove(&mut self, id: &str, name: &str) {
        let remove = if let Some(funcs) = self.0.get_mut(name) {
            funcs.remove(id);
            funcs.len() == 0
        } else {
            false
        };

        if remove {
            self.0.remove(name);
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
