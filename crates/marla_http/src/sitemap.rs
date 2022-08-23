use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    HttpRequest, HttpResponse, Responder,
};
use anyhow::Result;
use derive_more::{Display, Error};
use marla_site::page::index::PageIndex;
use sitemap::{structs::UrlEntry, writer::SiteMapWriter};

#[derive(Debug, Display, Error)]
pub enum SitemapError {
    #[display(fmt = "building sitemap failed: {}", msg)]
    BuildError { msg: String },
    #[display(fmt = "query pages failed: {}", msg)]
    QueryError { msg: String },
    #[display(fmt = "building entry failed: {}", msg)]
    EntryError { msg: String },
}

impl error::ResponseError for SitemapError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            SitemapError::BuildError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            SitemapError::QueryError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            SitemapError::EntryError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/sitemap.xml")]
pub async fn sitemap_route(req: HttpRequest) -> Result<impl Responder, SitemapError> {
    let page_index = PageIndex::create_and_parse()
        .map_err(|e| SitemapError::QueryError { msg: e.to_string() })?;

    let mut sitemap_output = Vec::<u8>::new();
    let sitemap_writer = SiteMapWriter::new(&mut sitemap_output);
    let mut url_writer = sitemap_writer
        .start_urlset()
        .expect("can't write the buffer");

    let mut page_paths = page_index
        .pages
        .iter()
        .map(|p| p.path.clone())
        .collect::<Vec<String>>();
    page_paths.sort();
    page_paths.dedup();

    for page_path in page_paths {
        let url_entry = UrlEntry::builder()
            .loc(
                req.url_for("page", &[&page_path[1..]])
                    .map_err(|e| SitemapError::EntryError { msg: e.to_string() })?
                    .as_str()
                    .replace("http:", "https:"),
            )
            .changefreq(sitemap::structs::ChangeFreq::Monthly)
            .priority(0.5)
            .lastmod(
                page_index
                    .page_by_uri_path(&page_path, None)
                    .unwrap()
                    .last_modified_at
                    .into(),
            )
            .build()
            .map_err(|e| SitemapError::EntryError { msg: e.to_string() })?;

        url_writer
            .url(url_entry)
            .map_err(|e| SitemapError::EntryError { msg: e.to_string() })?;
    }

    url_writer
        .end()
        .map_err(|e| SitemapError::BuildError { msg: e.to_string() })?;

    let sitemap_str = std::str::from_utf8(&sitemap_output.clone())
        .map_err(|e| SitemapError::BuildError { msg: e.to_string() })?
        .to_string();

    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::xml())
        .body(sitemap_str))
}
