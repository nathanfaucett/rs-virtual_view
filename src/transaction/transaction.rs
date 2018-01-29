use serde_json::{Map, Value};
use fnv::FnvHashMap;

use super::{Order, Patch, RawView};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transaction {
    events: FnvHashMap<String, FnvHashMap<String, bool>>,
    removes: FnvHashMap<String, RawView>,
    patches: FnvHashMap<String, Vec<Patch>>,
}

impl Transaction {
    #[inline(always)]
    pub fn new() -> Self {
        Transaction {
            events: FnvHashMap::default(),
            removes: FnvHashMap::default(),
            patches: FnvHashMap::default(),
        }
    }

    #[inline]
    pub fn mount(&mut self, id: &str, view: RawView) {
        self.append(id.into(), Patch::Mount(view));
    }
    #[inline]
    pub fn unmount(&mut self, id: &str, view: RawView) {
        self.removes.insert(id.into(), view);
    }

    #[inline]
    pub fn insert(&mut self, id: &str, view_id: &str, index: usize, view: RawView) {
        self.append(id.into(), Patch::Insert(view_id.into(), index, view));
    }
    #[inline]
    pub fn replace(&mut self, id: &str, prev_view: RawView, next_view: RawView) {
        self.append(id.into(), Patch::Replace(prev_view, next_view));
    }
    #[inline]
    pub fn order(&mut self, id: &str, order: Order) {
        self.append(id.into(), Patch::Order(order));
    }
    #[inline]
    pub fn props(
        &mut self,
        id: &str,
        prev_props: Map<String, Value>,
        diff_props: Map<String, Value>,
    ) {
        self.append(id.into(), Patch::Props(prev_props, diff_props));
    }

    #[inline]
    pub fn remove(&mut self, id: &str, view: RawView) {
        self.removes.insert(id.into(), view);
    }

    #[inline]
    pub fn add_event(&mut self, id: &str, name: &str) {
        self.append_event(id.into(), name.into(), true);
    }
    #[inline]
    pub fn remove_event(&mut self, id: &str, name: &str) {
        self.append_event(id.into(), name.into(), false);
    }

    #[inline(always)]
    pub fn events(&self) -> &FnvHashMap<String, FnvHashMap<String, bool>> {
        &self.events
    }
    #[inline(always)]
    pub fn removes(&self) -> &FnvHashMap<String, RawView> {
        &self.removes
    }
    #[inline(always)]
    pub fn patches(&self) -> &FnvHashMap<String, Vec<Patch>> {
        &self.patches
    }

    #[inline]
    fn append(&mut self, id: String, patch: Patch) {
        if !self.patches.contains_key(&id) {
            self.patches.insert(id.clone(), Vec::new());
        }

        let array = self.patches.get_mut(&id).unwrap();
        array.push(patch);
    }

    #[inline]
    fn append_event(&mut self, id: String, name: String, value: bool) {
        if !self.events.contains_key(&id) {
            self.events.insert(id.clone(), FnvHashMap::default());
        }

        let map = self.events.get_mut(&id).unwrap();
        map.insert(name, value);
    }
}
