use juniper::{graphql_object, FieldResult};

use marla_core::config::SETTINGS;
use marla_site::page::{
    markdown::{markdown_to_page, path_to_content_path},
    Page,
};

use super::context::GQLContext;

// Queries represent the callable funcitons
pub struct Query;
#[graphql_object(context = GQLContext)]
impl Query {
    fn api_version() -> &'static str {
        let version = SETTINGS
            .read()
            .unwrap()
            .get_string("version")
            .unwrap_or("1.0".to_string())
            .to_owned();

        return Box::leak(version.into_boxed_str());
    }

    fn page(path: String) -> FieldResult<Page> {
        Ok(markdown_to_page(path_to_content_path(path))?)
    }
}
