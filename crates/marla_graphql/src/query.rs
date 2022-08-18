use juniper::{graphql_object, FieldResult};

use marla_core::config::version;
use marla_site::page::{
    markdown::{get_pages, markdown_to_page, path_to_content_path},
    Page,
};

use super::context::GQLContext;

// Queries represent the callable funcitons
pub struct Query;
#[graphql_object(context = GQLContext)]
impl Query {
    fn api_version() -> &'static str {
        return Box::leak(version().into_boxed_str());
    }

    fn page(path: String) -> FieldResult<Page> {
        Ok(markdown_to_page(path_to_content_path(path))?)
    }

    fn pages(sub_path: Option<String>) -> FieldResult<Vec<Page>> {
        Ok(get_pages(sub_path)?)
    }
}
