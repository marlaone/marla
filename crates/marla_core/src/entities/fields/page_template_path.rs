#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PageTemplatePath(String);

impl PageTemplatePath {
    pub fn new(path: String) -> Self {
        Self(path)
    }
}

impl Default for PageTemplatePath {
    fn default() -> Self {
        Self::new(String::from("page.html"))
    }
}
