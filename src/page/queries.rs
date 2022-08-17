use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;

use crate::graphql::server::graphql_endpoint;

use self::page_by_path::PageByPathPage;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graph/schema.graphql",
    query_path = "graph/queries/page.graphql",
    response_derives = "Debug"
)]
pub struct PageByPath;

pub async fn query_page_by_path(
    variables: page_by_path::Variables,
) -> Result<Option<PageByPathPage>, Box<dyn Error>> {
    let request_body = PageByPath::build_query(variables);

    let client = reqwest::Client::new();
    let res = client
        .post(graphql_endpoint())
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<page_by_path::ResponseData> = res.json().await?;

    match response_body.data {
        Some(data) => Ok(Some(data.page)),
        None => Ok(None),
    }
}
