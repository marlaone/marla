use log::warn;

use crate::page::{
    markdown::{get_pages, markdown_to_page, path_to_content_path},
    Page,
};

pub fn fetch_page(path: &String) -> Option<Page> {
    match markdown_to_page(path_to_content_path(path, None)) {
        Ok(page) => Some(page),
        Err(e) => {
            warn!("failed to fetch page: {}", e);
            None
        }
    }
}

pub fn fetch_pages(sub_path: Option<String>) -> Vec<Page> {
    match get_pages(sub_path) {
        Ok(pages) => pages,
        Err(e) => {
            warn!("failed to fetch pages: {}", e);
            Vec::new()
        }
    }
}
