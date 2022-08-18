use marla_core::{
    config::{graphql_host, graphql_port, http_host, http_port},
    logger::setup_logger,
};
use marla_graphql::server::serve_graphql_server;
use marla_http::server::serve_http_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    setup_logger();

    let http_server =
        tokio::spawn(async move { serve_http_server(&http_host(), http_port()).unwrap().await });

    let graphql_server = tokio::spawn(async move {
        serve_graphql_server(&graphql_host(), graphql_port())
            .unwrap()
            .await
    });

    http_server.await?.expect("http server should be running");

    graphql_server
        .await?
        .expect("graphql server should be running");

    Ok(())
}
