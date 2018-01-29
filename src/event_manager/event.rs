use std::any::Any;

use serde_json::{Map, Value};

pub trait Event: 'static + Any {
    fn name(&self) -> &String;
    fn data(&self) -> &Map<String, Value>;
    fn propagation(&self) -> bool;
    fn stop_propagation(&mut self);

    #[inline]
    fn get(&self, key: &str) -> Option<&Value> {
        self.data().get(key)
    }
    #[inline]
    fn contains_key(&self, key: &str) -> bool {
        self.data().contains_key(key)
    }
}
