use super::super::{Props, Transaction, View};
use super::Tree;

pub trait Node {
    fn id(&self) -> &String;
    fn parent_id(&self) -> &String;
    fn state(&self) -> Props;
    fn last_rendered_view(&self) -> Option<&View>;
    fn mount(&mut self, &Tree, &mut Transaction) -> View;
    fn update(&mut self, View, &Tree, &mut Transaction) -> View;
}
