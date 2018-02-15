use std::sync::Arc;

use super::super::Props;
use super::Renderer;

pub struct UpdaterInner {
    id: String,
    depth: usize,
    renderer: Renderer,
}

#[derive(Clone)]
pub struct Updater(Arc<UpdaterInner>);

impl Updater {
    #[inline]
    pub fn new(id: String, depth: usize, renderer: Renderer) -> Self {
        Updater(Arc::new(UpdaterInner {
            id: id,
            depth: depth,
            renderer: renderer,
        }))
    }

    #[inline]
    pub fn update<F>(&self, f: F)
    where
        F: Fn(&Props) -> Props,
    {
        self.0.renderer.update(self.0.id.clone(), self.0.depth, f)
    }

    #[inline]
    pub fn force_update(&self) {
        self.update(Clone::clone);
    }
}
