use std::any::Any;

use super::super::{Props, Transaction, View};
use super::{ComponentHandler, Renderer, ViewHandler};

pub trait Handler: 'static + Any {
    fn mount(&mut self, &mut Transaction) -> View;
    fn receive(&mut self, next_view: View);
    fn update(&mut self, prev_view: View, next_view: View);
}

impl Handler {
    #[inline]
    pub fn new(renderer: &Renderer, view: View) -> Box<Self> {
        if let Some(component) = view.component().map(Clone::clone) {
            Box::new(ComponentHandler::new(renderer, view, component)) as Box<Handler>
        } else {
            Box::new(ViewHandler::new(view)) as Box<Handler>
        }
    }
}
