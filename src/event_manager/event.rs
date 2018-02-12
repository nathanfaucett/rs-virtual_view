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

pub struct SimpleEvent {
    name: String,
    data: Map<String, Value>,
    propagation: bool,
}

impl SimpleEvent {
    #[inline]
    pub fn new<T>(name: T, data: Map<String, Value>) -> Self
    where
        T: ToString,
    {
        SimpleEvent {
            name: name.to_string(),
            data: data,
            propagation: true,
        }
    }
}

impl Event for SimpleEvent {
    #[inline(always)]
    fn name(&self) -> &String {
        &self.name
    }
    #[inline(always)]
    fn data(&self) -> &Map<String, Value> {
        &self.data
    }
    #[inline(always)]
    fn propagation(&self) -> bool {
        self.propagation
    }
    #[inline(always)]
    fn stop_propagation(&mut self) {
        self.propagation = false;
    }
}
