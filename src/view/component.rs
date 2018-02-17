use std::any::Any;

use super::super::{Props, Updater};
use super::{Children, View};

pub trait Component: 'static + Any {
    fn render(&self, updater: &Updater, state: &Props, props: &Props, children: &Children) -> View;

    #[inline(always)]
    fn name(&self) -> &'static str {
        "Unknown"
    }

    /// return the inital state of the component
    #[inline]
    fn initial_state(&self, _props: &Props) -> Props {
        Props::new()
    }

    /// called before mount
    #[inline(always)]
    fn will_mount(&self) {}

    /// called before unmount
    #[inline(always)]
    fn will_unmount(&self) {}

    /// called before update
    #[inline(always)]
    fn will_update(&self) {}

    /// called when component receives new state, props, or children
    #[inline(always)]
    fn receive_props(&self, _state: &Props, _props: &Props, _children: &Children) {}

    /// if component needs update return true, defaults to true
    #[inline]
    fn should_update(
        &self,
        _prev_state: &Props,
        _prev_props: &Props,
        _prev_children: &Children,

        _next_state: &Props,
        _next_props: &Props,
        _next_children: &Children,
    ) -> bool {
        true
    }
}

impl PartialEq for Component {
    #[inline]
    fn eq(&self, other: &Component) -> bool {
        self.get_type_id() == other.get_type_id()
    }
}
