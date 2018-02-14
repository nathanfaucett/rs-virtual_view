use super::super::Props;
use super::Renderer;

#[derive(Clone)]
pub struct Updater {
    renderer: Renderer,
}

impl Updater {
    #[inline]
    pub fn new(renderer: Renderer) -> Self {
        Updater { renderer: renderer }
    }

    #[inline]
    pub fn update<F>(&self, f: F)
    where
        F: Fn(&Props) -> Props,
    {

    }

    #[inline]
    pub fn force_update(&self) {
        self.update(Clone::clone);
    }
}
