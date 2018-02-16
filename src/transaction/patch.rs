use serde_json::{Map, Value};

use super::{Order, RawView};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Patch {
    Mount(RawView),
    Insert(String, usize, RawView),
    Replace(RawView, RawView),
    Order(Order),
    Props(Map<String, Value>, Map<String, Value>),
}

impl Patch {
    #[inline]
    pub fn is_mount(&self) -> bool {
        match self {
            &Patch::Mount(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_insert(&self) -> bool {
        match self {
            &Patch::Insert(_, _, _) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_replace(&self) -> bool {
        match self {
            &Patch::Replace(_, _) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_order(&self) -> bool {
        match self {
            &Patch::Order(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_props(&self) -> bool {
        match self {
            &Patch::Props(_, _) => true,
            _ => false,
        }
    }
}
