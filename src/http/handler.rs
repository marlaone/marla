use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    HttpRequest, HttpResponse, Responder,
};
use derive_more::{Display, Error};

use crate::page::queries::query_page_by_path;

#[derive(Debug, Display, Error)]
pub enum PageError {
    #[display(fmt = "Validation error on field: {}", msg)]
    QueryError { msg: String },
    #[display(fmt = "page not found")]
    PageNotFound,
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
        }
    }
}

#[get("/{path:.*}")]
pub async fn index(req: HttpRequest) -> Result<impl Responder, PageError> {
    let potential_page = query_page_by_path(crate::page::queries::page_by_path::Variables {
        path: req.uri().path().to_string(),
    })
    .await
    .map_err(|e| PageError::QueryError { msg: e.to_string() })?;

    match potential_page {
        Some(page) => Ok(page.content),
        None => Err(PageError::PageNotFound),
    }
}
