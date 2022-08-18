use marla_core::{
    config::{http_host, http_port},
    logger::setup_logger,
};
use marla_http::server::serve_http_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    setup_logger();

    serve_http_server(&http_host(), http_port())
        .unwrap()
        .await?;

    Ok(())
}
