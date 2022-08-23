use std::path::PathBuf;

use anyhow::Result;
use glob::glob;
use marla_core::config::site_content_path;
use path_clean::PathClean;

use super::{markdown::markdown_to_page, Page};

#[derive(Debug)]
pub struct PageIndex {
    pub pages: Vec<Page>,
}

impl PageIndex {
    pub fn new() -> PageIndex {
        PageIndex { pages: Vec::new() }
    }

    pub fn create_and_parse() -> Result<PageIndex> {
        let mut page_index = PageIndex {
            pages: Vec::with_capacity(0),
        };
        page_index.parse()?;
        return Ok(page_index);
    }

    pub fn parse(&mut self) -> Result<()> {
        self.pages = get_pages()?;

        Ok(())
    }

    pub fn page_by_content_path(
        &self,
        content_path: &PathBuf,
        lang: Option<&String>,
    ) -> Option<&Page> {
        let cleaned_path = content_path.clean();
        match self
            .pages
            .iter()
            .find(|p| p.content_path == cleaned_path && p.lang.as_ref() == lang)
        {
            Some(page) => Some(page),
            None => self
                .pages
                .iter()
                .find(|p| p.content_path == cleaned_path && p.lang == None),
        }
    }

    pub fn page_by_uri_path(&self, uri_path: &String, lang: Option<&String>) -> Option<&Page> {
        match self
            .pages
            .iter()
            .find(|p| &p.path == uri_path && p.lang.as_ref() == lang)
        {
            Some(page) => Some(page),
            None => self
                .pages
                .iter()
                .find(|p| &p.path == uri_path && p.lang == None),
        }
    }
}

fn get_pages() -> Result<Vec<Page>> {
    let mut pages = Vec::new();

    for &ext in ["md", "html"].iter() {
        let mut contents_path = site_content_path();

        contents_path.push_str("/**/*.");
        contents_path.push_str(ext);

        for content_entry in glob(&contents_path)? {
            match content_entry {
                Ok(page_path) => pages.push(markdown_to_page(page_path)?),
                Err(e) => log::error!("{:?}", e),
            }
        }
    }

    pages.sort_by(|a, b| a.path.partial_cmp(&b.path).unwrap());

    Ok(pages)
}
