use website_rs::config::SETTINGS;
use website_rs::graphql::server::serve_graphql_server;
use website_rs::logger::setup_logger;

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
