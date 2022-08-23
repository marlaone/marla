use std::sync::Arc;

use actix_web::{
    dev::Server,
    middleware::{Compress, Logger, NormalizePath},
    web, App, HttpServer,
};
use anyhow::Result;
use log::info;
use marla_site::{page::index::PageIndex, theme::Theme};
use tokio::sync::{Mutex, RwLock};

use crate::{handler::page, sitemap::sitemap_route};

pub async fn serve_http_server(host: &str, port: u16) -> Result<Server> {
    let theme = Theme::new()?;

    let page_index = Arc::new(RwLock::new(PageIndex::new()));
    page_index.write().await.parse()?;
    PageIndex::watch(page_index.clone());

    info!("Listening on http://{}:{}", host, port);
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(theme.clone())))
            .app_data(web::Data::from(page_index.clone()))
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(sitemap_route)
            .service(page)
    })
    .workers(num_cpus::get())
    .bind((host, port))?
    .run())
}
