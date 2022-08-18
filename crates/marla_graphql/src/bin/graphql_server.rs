use marla_core::{config::SETTINGS, logger::setup_logger};
use marla_graphql::server::serve_graphql_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    setup_logger();

    let graphql_host = SETTINGS
        .read()
        .unwrap()
        .get::<String>("graphql.host")
        .unwrap_or("localhost".to_string());
    let graphql_port = SETTINGS
        .read()
        .unwrap()
        .get::<u16>("graphql.port")
        .unwrap_or(1808);
    serve_graphql_server(&graphql_host, graphql_port)
        .unwrap()
        .await?;

    Ok(())
}
