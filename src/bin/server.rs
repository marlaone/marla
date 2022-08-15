use website_rs::config::SETTINGS;
use website_rs::logger::setup_logger;
use website_rs::server::serve_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    setup_logger();

    serve_server(
        SETTINGS
            .read()
            .unwrap()
            .get::<String>("http.host")
            .unwrap_or("localhost".to_string())
            .as_str(),
        SETTINGS
            .read()
            .unwrap()
            .get::<u16>("http.port")
            .unwrap_or(1808),
    )
    .unwrap()
    .await
}
