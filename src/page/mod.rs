use juniper::graphql_object;

use crate::graphql::context::GQLContext;

use self::meta::PageMeta;

pub mod meta;
pub mod queries;

#[derive(Clone, Debug)]
pub struct Page {
    pub meta: Option<PageMeta>,
    pub content: String,
}

#[graphql_object(context = GQLContext)]
impl Page {
    fn meta(&self) -> Option<&PageMeta> {
        self.meta.as_ref()
    }
    fn content(&self) -> &str {
        &self.content
    }
}
