#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PageMeta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
}

impl Default for PageMeta {
    fn default() -> Self {
        PageMeta {
            title: None,
            description: None,
            keywords: None,
        }
    }
}
