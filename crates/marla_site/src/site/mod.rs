use tera::Value;

use crate::{
    data::get_site_data,
    page::{index::PageIndex, Page},
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
    pub async fn from_uri_path(
        uri_path: String,
        lang_tag: Option<&String>,
    ) -> Result<Site, Box<dyn std::error::Error>> {
        let mut page_index = PageIndex::new();
        page_index.parse()?;

        let data = get_site_data()?;

        Ok(Site {
            page: page_index
                .page_by_uri_path(&uri_path, lang_tag)
                .map(|p| p.to_owned()),
            path: uri_path,
            pages: page_index.pages,
            data,
            lang: lang_tag.map(|lt| lt.to_owned()),
        })
    }
}
