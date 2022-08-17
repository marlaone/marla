use website_rs::config::SETTINGS;
use website_rs::graphql::server::serve_graphql_server;
use website_rs::http::server::serve_http_server;
use website_rs::logger::setup_logger;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    setup_logger();

    let http_host = SETTINGS
        .read()
        .unwrap()
        .get::<String>("http.host")
        .unwrap_or("localhost".to_string());
    let http_port = SETTINGS
        .read()
        .unwrap()
        .get::<u16>("http.port")
        .unwrap_or(1809);
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

    println!(
        "logger level = {}",
        SETTINGS.read().unwrap().get_string("logger.level").unwrap()
    );

    let http_server =
        tokio::spawn(async move { serve_http_server(&http_host, http_port).unwrap().await });

    let graphql_server = tokio::spawn(async move {
        serve_graphql_server(&graphql_host, graphql_port)
            .unwrap()
            .await
    });

    http_server.await?.expect("http server should be running");

    graphql_server
        .await?
        .expect("graphql server should be running");

    Ok(())
}
