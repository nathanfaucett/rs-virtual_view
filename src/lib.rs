#![feature(get_type_id)]

extern crate fnv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod diff;
mod event_manager;
#[macro_use]
mod prop;
mod renderer;
mod transaction;
mod utils;
#[macro_use]
pub mod view;

pub use self::diff::{diff_children, diff_props, diff_props_object, DiffChildren};
pub use self::event_manager::{Event, EventManager, SimpleEvent};
pub use self::prop::{array_to_json, prop_to_json, props_to_json, Array, Function, Number, Prop,
                     Props};
pub use self::renderer::{Renderer, Updater};
pub use self::transaction::{Order, Patch, Transaction};
pub use self::utils::{child_view_id, is_ancestor_id_of, is_boundary, next_descendant_id,
                      parent_id, traverse_path, view_id};
pub use self::view::{Children, Component, View, ViewKind};
