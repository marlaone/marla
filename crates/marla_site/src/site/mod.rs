use tera::Value;

use crate::{
    data::get_site_data,
    page::Page,
    services::page::{fetch_page, fetch_pages},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Site {
    pub path: String,
    pub page: Option<Page>,
    pub pages: Vec<Page>,
    pub data: Value,
    pub lang: Option<String>,
}

impl Site {
    pub async fn from_content_path(
        content_path: String,
        lang_tag: Option<&String>,
    ) -> Result<Site, Box<dyn std::error::Error>> {
        let potential_page = fetch_page(&content_path, lang_tag);

        let mut page = None;
        match potential_page {
            Some(found_page) => page = Some(found_page),
            None => {}
        };

        let pages = fetch_pages(None, lang_tag);

        let data = get_site_data()?;

        Ok(Site {
            path: content_path,
            page,
            pages,
            data,
            lang: lang_tag.map(|lt| lt.to_owned()),
        })
    }
}
