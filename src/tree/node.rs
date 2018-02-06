use super::super::{EventManager, Transaction, View};
use super::Nodes;

pub trait Node {
    fn id(&self) -> &String;
    fn parent_id(&self) -> &String;
    fn last_rendered_view(&self) -> Option<&View>;
    fn mount(&mut self, &Nodes, &EventManager, &mut Transaction) -> View;
    fn update(&mut self, View, &Nodes, &EventManager, &mut Transaction) -> View;
}
