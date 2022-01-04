use std::any::Any;

use super::super::{Instance, Props};
use super::{Children, View};

pub trait Component: 'static + Any {
    fn render(&self, instance: &Instance, props: &Props, children: &Children) -> View;

    #[inline(always)]
    fn name(&self) -> &'static str {
        "Unknown"
    }

    /// return the inital state of the component
    #[inline]
    fn initial_state(&self, _props: &Props) -> Props {
        Props::new()
    }

    /// return the inital context of the component, gets passed to children via inherit_context
    #[inline]
    fn context(&self, _props: &Props) -> Props {
        Props::new()
    }
    /// filter the passed context of the component
    #[inline]
    fn inherit_context(&self, context: Props, _parent_context: &Props) -> Props {
        context
    }

    /// called before mount
    #[inline(always)]
    fn will_mount(&self, _instance: &Instance) {}

    /// called before unmount
    #[inline(always)]
    fn will_unmount(&self, _instance: &Instance) {}

    /// called before update
    #[inline(always)]
    fn will_update(&self, _instance: &Instance) {}

    /// called when component receives new state, props, or children
    #[inline(always)]
    fn receive_props(
        &self,
        _instance: &Instance,
        _next_state: &Props,
        _next_props: &Props,
        _next_children: &Children,
    ) {
    }

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

impl PartialEq for dyn Component {
    #[inline]
    fn eq(&self, other: &dyn Component) -> bool {
        self.type_id() == other.type_id()
    }
}
