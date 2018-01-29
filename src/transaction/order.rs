#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Order {
    removes: Vec<(usize, Option<String>)>,
    inserts: Vec<(Option<String>, usize)>,
}

impl Order {
    #[inline(always)]
    pub fn new(
        removes: Vec<(usize, Option<String>)>,
        inserts: Vec<(Option<String>, usize)>,
    ) -> Self {
        Order {
            removes: removes,
            inserts: inserts,
        }
    }

    #[inline(always)]
    pub fn removes(&self) -> &[(usize, Option<String>)] {
        &*self.removes
    }
    #[inline(always)]
    pub fn inserts(&self) -> &[(Option<String>, usize)] {
        &*self.inserts
    }
}
