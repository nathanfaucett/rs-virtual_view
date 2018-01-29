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
