use super::super::Props;
use super::Tree;

#[derive(Clone)]
pub struct Updater {
    id: String,
    tree: Tree,
}

impl Updater {
    #[inline]
    pub(super) fn new(id: String, tree: Tree) -> Self {
        Updater { id: id, tree: tree }
    }

    #[inline]
    pub fn update<F>(&self, f: F)
    where
        F: Fn(&mut Props),
    {
        self.tree.update(&self.id, f);
    }
}
