use serde_json::Value;

use super::super::Props;
use super::Updater;

pub struct Instance {
    pub state: Props,
    pub context: Props,
    pub updater: Updater,
}

impl Instance {
    #[inline]
    pub(super) fn new(state: Props, context: Props, updater: Updater) -> Self {
        Instance {
            state: state,
            context: context,
            updater: updater,
        }
    }

    #[inline]
    pub fn send<N, V, F>(&self, name: N, json: V, f: F)
    where
        N: Into<String>,
        V: Into<Value>,
        F: 'static + Fn(Value),
    {
        self.updater.send(name, json, f)
    }

    #[inline]
    pub fn send_no_callback<N, V>(&self, name: N, json: V)
    where
        N: Into<String>,
        V: Into<Value>,
    {
        self.updater.send_no_callback(name, json)
    }

    #[inline(always)]
    pub fn state(&self) -> &Props {
        &self.state
    }
    #[inline(always)]
    pub fn context(&self) -> &Props {
        &self.context
    }
    #[inline(always)]
    pub fn updater(&self) -> &Updater {
        &self.updater
    }

    #[inline]
    pub fn set_state<F>(&self, f: F)
    where
        F: 'static + Send + Fn(&Props) -> Props,
    {
        self.updater.set_state(f)
    }

    #[inline]
    pub fn force_update(&self) {
        self.updater.force_update()
    }
}
