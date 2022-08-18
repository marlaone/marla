use marla_core::{
    config::{graphql_host, graphql_port},
    logger::setup_logger,
};
use marla_graphql::server::serve_graphql_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    setup_logger();

    serve_graphql_server(&graphql_host(), graphql_port())
        .unwrap()
        .await?;

    Ok(())
}
