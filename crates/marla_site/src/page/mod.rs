use chrono::{DateTime, Utc};

use self::meta::PageMeta;

pub mod markdown;
pub mod meta;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Page {
    pub path: String,
    pub meta: Option<PageMeta>,
    pub content: String,
    pub plain: String,
    pub last_modified_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub params: toml::value::Table,
    pub words: usize,
}
