use actix_cors::Cors;
use actix_web::{
    dev::Server,
    http::header,
    middleware::{Compress, Logger, NormalizePath},
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Result;
use log::debug;

use crate::graphql::{
    handler::{graphiql_route, graphql_route},
    schema::schema,
};

pub fn serve_server(host: &str, port: u16) -> Result<Server> {
    debug!("Listening on http://{}:{}", host, port);
    Ok(HttpServer::new(|| {
        App::new()
            .app_data(Data::new(schema()))
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
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(graphiql_route)))
    })
    .workers(num_cpus::get())
    .bind((host, port))?
    .run())
}
