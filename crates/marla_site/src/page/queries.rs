use chrono::{DateTime, Utc};
use graphql_client::{GraphQLQuery, Response};
use marla_core::config::{graphql_host, graphql_port, graphql_protocol};
use reqwest;
use std::{error::Error, time::Duration};

use self::{all_pages::AllPagesPages, page_by_path::PageByPathPage};

type DateTimeUtc = DateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graph/schema.graphql",
    query_path = "graph/queries/page.graphql",
    response_derives = "Debug,serde::Serialize,Clone,Default"
)]
pub struct PageByPath;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graph/schema.graphql",
    query_path = "graph/queries/pages.graphql",
    response_derives = "Debug,serde::Serialize,Clone,Default"
)]
pub struct AllPages;

#[derive(Debug, Clone)]
pub struct PageClient {
    cache: moka::future::Cache<String, PageByPathPage>,
}

impl PageClient {
    pub fn new() -> PageClient {
        PageClient {
            cache: moka::future::Cache::builder()
                .max_capacity(10_000)
                .time_to_live(Duration::from_secs(3600))
                .time_to_idle(Duration::from_secs(5 * 60))
                .build(),
        }
    }

    pub async fn query_pages(
        &self,
        sub_path: Option<String>,
    ) -> Result<Option<Vec<AllPagesPages>>, Box<dyn Error>> {
        let request_body = AllPages::build_query(all_pages::Variables { sub_path });

        let client = reqwest::Client::new();
        let res = client
            .post(graphql_endpoint())
            .json(&request_body)
            .send()
            .await?;
        let response_body: Response<all_pages::ResponseData> = res.json().await?;

        match response_body.data {
            Some(data) => Ok(Some(data.pages)),
            None => Ok(None),
        }
    }

    pub async fn query_page_by_path(
        &self,
        path: String,
    ) -> Result<Option<PageByPathPage>, Box<dyn Error>> {
        let variables = page_by_path::Variables { path };
        let cache_key = variables.path.clone();
        let cached_page = self.cache.get(&cache_key);
        if cached_page.is_some() {
            return Ok(Some(cached_page.unwrap()));
        }

        let request_body = PageByPath::build_query(variables);

        let client = reqwest::Client::new();
        let res = client
            .post(graphql_endpoint())
            .json(&request_body)
            .send()
            .await?;
        let response_body: Response<page_by_path::ResponseData> = res.json().await?;

        match response_body.data {
            Some(data) => {
                self.cache.insert(cache_key, data.page.clone()).await;

                Ok(Some(data.page))
            }
            None => Ok(None),
        }
    }
}

pub fn graphql_endpoint() -> String {
    format!(
        "{}://{}:{}/graphql",
        graphql_protocol(),
        graphql_host(),
        graphql_port()
    )
}
