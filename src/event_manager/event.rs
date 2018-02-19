use std::any::Any;

use serde_json::{Map, Value};

pub trait Event: 'static + Any {
    fn name(&self) -> &String;

    fn target_id(&self) -> &String;
    fn set_target_id(&mut self, target_id: String);

    fn data(&self) -> &Map<String, Value>;
    fn data_mut(&mut self) -> &mut Map<String, Value>;

    fn propagation(&self) -> bool;
    fn stop_propagation(&mut self);

    #[inline]
    fn get(&self, key: &str) -> Option<&Value> {
        self.data().get(key)
    }
    #[inline]
    fn set(&mut self, key: String, value: Value) {
        self.data_mut().insert(key, value);
    }
    #[inline]
    fn contains_key(&self, key: &str) -> bool {
        self.data().contains_key(key)
    }
}

pub struct SimpleEvent {
    name: String,
    target_id: String,
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
            target_id: String::new(),
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
    fn target_id(&self) -> &String {
        &self.target_id
    }
    #[inline(always)]
    fn set_target_id(&mut self, target_id: String) {
        self.target_id = target_id;
    }
    #[inline(always)]
    fn data(&self) -> &Map<String, Value> {
        &self.data
    }
    #[inline(always)]
    fn data_mut(&mut self) -> &mut Map<String, Value> {
        &mut self.data
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
