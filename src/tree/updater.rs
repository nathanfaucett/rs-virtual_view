#[derive(Clone, Copy)]
pub struct Updater;

unsafe impl Send for Updater {}
unsafe impl Sync for Updater {}

impl Updater {
    #[inline(always)]
    pub fn new() -> Self {
        Updater
    }
}
