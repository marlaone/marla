use actix_cors::Cors;
use actix_web::{
    dev::Server,
    http::header,
    middleware::{Compress, Logger, NormalizePath},
    web, App, HttpServer,
};
use anyhow::Result;
use log::debug;
use marla_site::{page::queries::PageClient, theme::Theme};
use tokio::sync::Mutex;

use crate::handler::page;

pub fn serve_http_server(host: &str, port: u16) -> Result<Server> {
    let theme = Theme::new()?;
    let page_client = PageClient::new();

    debug!("Listening on http://{}:{}", host, port);
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(theme.clone())))
            .app_data(web::Data::new(page_client.clone()))
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .expose_headers(vec![header::LINK])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(page)
    })
    .workers(num_cpus::get())
    .bind((host, port))?
    .run())
}
