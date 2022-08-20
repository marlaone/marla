use actix_cors::Cors;
use actix_web::{
    dev::Server,
    http::header,
    middleware::{Compress, Logger, NormalizePath},
    web, App, HttpServer,
};
use anyhow::Result;
use log::info;
use marla_site::theme::Theme;
use tokio::sync::Mutex;

use crate::{handler::page, sitemap::sitemap_route};

pub fn serve_http_server(host: &str, port: u16) -> Result<Server> {
    let theme = Theme::new()?;

    info!("Listening on http://{}:{}", host, port);
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(theme.clone())))
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
            .service(sitemap_route)
            .service(page)
    })
    .workers(num_cpus::get())
    .bind((host, port))?
    .run())
}
