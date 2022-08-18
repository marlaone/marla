use graphql_client::{GraphQLQuery, Response};
use marla_core::config::SETTINGS;
use reqwest;
use std::{error::Error, time::Duration};

use self::page_by_path::PageByPathPage;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graph/schema.graphql",
    query_path = "graph/queries/page.graphql",
    response_derives = "Debug,serde::Serialize,Clone"
)]
pub struct PageByPath;

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

    pub async fn query_page_by_path(
        &self,
        variables: page_by_path::Variables,
    ) -> Result<Option<PageByPathPage>, Box<dyn Error>> {
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
    let prot = SETTINGS
        .read()
        .unwrap()
        .get_string("graphql.protocol")
        .unwrap_or("http".to_string());
    let host = SETTINGS
        .read()
        .unwrap()
        .get_string("graphql.host")
        .unwrap_or("localhost".to_string());
    let port = SETTINGS
        .read()
        .unwrap()
        .get::<u16>("graphql.port")
        .unwrap_or(1808);
    format!("{}://{}:{}/graphql", prot, host, port)
}
