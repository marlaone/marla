use marla_core::{config::SETTINGS, logger::setup_logger};
use marla_http::server::serve_http_server;

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

    serve_http_server(&http_host, http_port).unwrap().await?;

    Ok(())
}
