use super::{Prop, Props};

pub type Function = dyn Fn(&mut Props) -> Prop;
