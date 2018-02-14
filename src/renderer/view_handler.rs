use super::super::{Props, Transaction, View};
use super::Handler;

pub struct ViewHandler {
    view: View,
    rendered_view: View,
}

impl ViewHandler {
    #[inline]
    pub fn new(view: View) -> Self {
        ViewHandler {
            view: view,
            rendered_view: View::new_empty(),
        }
    }
}

impl Handler for ViewHandler {
    #[inline]
    fn mount(&mut self, transaction: &mut Transaction) -> View {
        self.view.clone()
    }

    #[inline]
    fn receive(&mut self, next_view: View) {}

    #[inline]
    fn update(&mut self, prev_view: View, next_view: View) {}
}
