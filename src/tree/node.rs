use super::super::{EventManager, Transaction, View};
use super::Nodes;

pub trait Node {
    fn id(&self) -> &String;
    fn parent_id(&self) -> &String;
    fn mount(&mut self, &Nodes, &mut Transaction, &mut EventManager) -> View;
}
