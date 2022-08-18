use juniper::GraphQLObject;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, GraphQLObject)]
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
