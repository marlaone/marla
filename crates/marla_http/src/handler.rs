use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use actix_files::NamedFile;
use actix_web::{
    body::MessageBody,
    error, get,
    http::{header::ContentType, StatusCode},
    web, HttpRequest, HttpResponse, Responder,
};
use derive_more::{Display, Error};
use marla_core::config::site_output_path;
use marla_site::{
    page::{markdown::path_to_content_path, queries::PageClient},
    site::Site,
    theme::{get_theme_path, Theme},
};
use tokio::sync::Mutex;

#[derive(Debug, Display, Error)]
pub enum PageError {
    #[display(fmt = "querying page failed: {}", msg)]
    QueryError { msg: String },
    #[display(fmt = "page not found")]
    PageNotFound,
    #[display(fmt = "rendering page failed: {}", msg)]
    RenderError { msg: String },
    #[display(fmt = "failed to load static file: {}", msg)]
    StaticError { msg: String },
    #[display(fmt = "something went wrong: {}", msg)]
    ServiceFailure { msg: String },
    #[display(fmt = "failed to construct site object: {}", msg)]
    SiteError { msg: String },
}

impl error::ResponseError for PageError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            PageError::QueryError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            PageError::PageNotFound => StatusCode::NOT_FOUND,
            PageError::RenderError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            PageError::StaticError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            PageError::ServiceFailure { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            PageError::SiteError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

fn check_static_file(path: &str, req: &HttpRequest) -> Result<Option<HttpResponse>, PageError> {
    let static_file_path = current_dir()
        .map_err(|e| PageError::ServiceFailure { msg: e.to_string() })?
        .join(&path);
    if Path::new(&static_file_path).is_file() {
        return Ok(Some(
            NamedFile::open(static_file_path)
                .map_err(|e| PageError::ServiceFailure { msg: e.to_string() })
                .respond_to(req)
                .map_body(|_, b| b.boxed()),
        ));
    }
    return Ok(None);
}

fn serve_static_files(req: &HttpRequest) -> Result<Option<HttpResponse>, PageError> {
    let mut relative_file_path = "".to_string();
    if !req
        .uri()
        .path()
        .to_string()
        .replace("..", ".")
        .starts_with(".")
    {
        relative_file_path.push_str(".");
    }
    relative_file_path.push_str(req.uri().path().to_string().replace("..", ".").as_str());

    let theme_path =
        get_theme_path().map_err(|e| PageError::ServiceFailure { msg: e.to_string() })?;

    let output_path = site_output_path();
    let mut static_pathes = vec![
        PathBuf::from(&theme_path).join("./content"),
        PathBuf::from(&theme_path).join("./static"),
        PathBuf::from("./site/static"),
    ];

    if output_path != "" {
        static_pathes.push(PathBuf::from(&theme_path).join(output_path));
    }

    for path in static_pathes.iter() {
        let another_static_path = path.join(&relative_file_path);
        match check_static_file(another_static_path.to_str().unwrap(), &req)? {
            Some(res) => return Ok(Some(res)),
            None => (),
        }
    }

    Ok(None)
}

async fn serve_html_template(
    theme: web::Data<Mutex<Theme>>,
    site: &Site,
) -> Result<Option<HttpResponse>, PageError> {
    let content_path = path_to_content_path(&site.path, Some(".html".to_string()));

    return Ok(if content_path.exists() {
        Some(
            HttpResponse::build(StatusCode::OK)
                .insert_header(ContentType::html())
                .body(
                    theme
                        .lock()
                        .await
                        .render_template(content_path.to_str().unwrap_or_default(), &site)
                        .map_err(|e| PageError::RenderError { msg: e.to_string() })?,
                ),
        )
    } else {
        None
    });
}

#[get("/{path:.*}", name = "page")]
pub async fn page(
    req: HttpRequest,
    theme: web::Data<Mutex<Theme>>,
    page_client: web::Data<PageClient>,
) -> Result<impl Responder, PageError> {
    match serve_static_files(&req)? {
        Some(res) => return Ok(res),
        None => (),
    }

    let site = Site::from_content_path(req.uri().path().to_string(), &page_client)
        .await
        .map_err(|e| PageError::SiteError { msg: e.to_string() })?;

    match serve_html_template(theme.clone(), &site).await? {
        Some(res) => return Ok(res),
        None => (),
    }

    match &site.page {
        Some(_) => Ok(HttpResponse::build(StatusCode::OK)
            .insert_header(ContentType::html())
            .body(
                theme
                    .lock()
                    .await
                    .render_page(&site)
                    .map_err(|e| PageError::RenderError { msg: e.to_string() })?,
            )),
        None => Err(PageError::PageNotFound),
    }
}
