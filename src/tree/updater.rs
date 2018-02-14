use super::super::Props;
use super::Tree;

#[derive(Clone)]
pub struct Updater {
    id: String,
    depth: usize,
    tree: Tree,
}

impl Updater {
    #[inline]
    pub(super) fn new(id: String, depth: usize, tree: Tree) -> Self {
        Updater {
            id: id,
            depth: depth,
            tree: tree,
        }
    }

    #[inline]
    pub fn update<F>(&self, f: F)
    where
        F: Fn(&Props) -> Props,
    {
        self.tree.update(self.id.clone(), self.depth, f);
    }

    #[inline]
    pub fn force_update(&self) {
        self.update(Clone::clone);
    }
}
