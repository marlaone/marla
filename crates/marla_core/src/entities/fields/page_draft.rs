#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PageDraft(bool);

impl PageDraft {
    pub fn new(draft: bool) -> Self {
        Self(draft)
    }
}

impl Default for PageDraft {
    fn default() -> Self {
        Self(false)
    }
}
