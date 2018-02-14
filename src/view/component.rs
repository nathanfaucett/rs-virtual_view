use std::any::Any;

use super::super::{Props, Updater};
use super::{Children, View};

pub trait Component: 'static + Any {
    fn render(&self, updater: &Updater, state: &Props, props: &Props, children: &Children) -> View;

    #[inline(always)]
    fn name(&self) -> &'static str {
        "Unknown"
    }

    #[inline]
    fn initial_state(&self, _props: &Props) -> Props {
        Props::new()
    }

    #[inline(always)]
    fn will_mount(&self, updater: &Updater) {}
    #[inline(always)]
    fn will_unmount(&self) {}
    #[inline(always)]
    fn will_update(&self) {}

    #[inline(always)]
    fn receive_props(&self, _state: Props, _props: &Props, _children: &Children) {}

    #[inline]
    fn should_update(
        &self,
        prev_state: &Props,
        prev_props: &Props,
        prev_children: &Children,

        next_state: &Props,
        next_props: &Props,
        next_children: &Children,
    ) -> bool {
        !(prev_state == next_state && prev_props == next_props && prev_children == next_children)
    }
}
