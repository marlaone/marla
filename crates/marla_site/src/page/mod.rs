use chrono::{DateTime, Utc};
use juniper::graphql_object;

use self::meta::PageMeta;

pub mod markdown;
pub mod meta;
pub mod queries;

#[derive(Clone, Debug)]
pub struct Page {
    pub path: String,
    pub meta: Option<PageMeta>,
    pub content: String,
    pub last_modified_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub params: toml::value::Table,
}

#[graphql_object]
impl Page {
    fn path(&self) -> &str {
        &self.path
    }
    fn meta(&self) -> Option<&PageMeta> {
        self.meta.as_ref()
    }
    fn content(&self) -> &str {
        &self.content
    }
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    fn last_modified_at(&self) -> DateTime<Utc> {
        self.last_modified_at
    }
}
