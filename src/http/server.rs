use actix_cors::Cors;
use actix_web::{
    dev::Server,
    http::header,
    middleware::{Compress, Logger, NormalizePath},
    App, HttpServer,
};
use anyhow::Result;
use log::debug;

use crate::http::handler::index;

pub fn serve_http_server(host: &str, port: u16) -> Result<Server> {
    debug!("Listening on http://{}:{}", host, port);
    Ok(HttpServer::new(|| {
        App::new()
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
            .service(index)
    })
    .workers(num_cpus::get())
    .bind((host, port))?
    .run())
}
