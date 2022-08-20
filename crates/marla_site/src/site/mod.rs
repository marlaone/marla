use tera::Value;

use crate::{
    data::get_site_data,
    page::queries::{all_pages::AllPagesPages, page_by_path::PageByPathPage, PageClient},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Site {
    pub path: String,
    pub page: Option<PageByPathPage>,
    pub pages: Vec<AllPagesPages>,
    pub data: Value,
}

impl Site {
    pub async fn from_content_path(
        content_path: String,
        page_client: &PageClient,
    ) -> Result<Site, Box<dyn std::error::Error>> {
        let potential_page = page_client.query_page_by_path(&content_path).await?;

        let mut page = None;
        match potential_page {
            Some(found_page) => page = Some(found_page),
            None => {}
        };

        let pages = page_client.query_pages(None).await?.unwrap_or_default();

        let data = get_site_data()?;

        Ok(Site {
            path: content_path,
            page,
            pages,
            data,
        })
    }
}
