use std::any::Any;

use super::super::Updater;
use super::{Children, Props, View};

pub trait Component: 'static + Any {
    fn render(&self, updater: Updater, state: &Props, props: &Props, children: &Children) -> View;

    #[inline(always)]
    fn name(&self) -> &'static str {
        "Component"
    }

    #[inline]
    fn initial_state(&self) -> Props {
        Props::default()
    }

    #[inline(always)]
    fn mount(&self) {}
    #[inline(always)]
    fn unmount(&self) {}
    #[inline(always)]
    fn update(&self) {}
    #[inline(always)]
    fn receive_props(&self, _state: Props, _props: &Props, _children: &Children) {}

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
