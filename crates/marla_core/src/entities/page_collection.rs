use super::page::Page;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PageCollection {
    pub pages: Vec<Page>,
}
