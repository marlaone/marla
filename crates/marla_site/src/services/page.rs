use std::path::PathBuf;

use log::warn;

use crate::page::{
    markdown::{get_pages, markdown_to_page, path_to_content_path},
    Page,
};

pub fn has_language_page(path: &PathBuf, lang_tag: &String) -> bool {
    return get_langauge_page_path(path, lang_tag).exists();
}

pub fn get_langauge_page_path(path: &PathBuf, lang_tag: &String) -> PathBuf {
    let file_ext = path.as_path().extension().unwrap_or_default();
    let mut file_path = path
        .as_path()
        .with_extension("")
        .to_str()
        .unwrap_or_default()
        .to_string();

    file_path.push_str(".");
    file_path.push_str(lang_tag);
    file_path.push_str(".");
    file_path.push_str(file_ext.to_str().unwrap_or_default());

    return PathBuf::from(file_path);
}

pub fn get_page_path(path: PathBuf, lang_tag: Option<&String>) -> PathBuf {
    match lang_tag {
        Some(lang_tag) => {
            if has_language_page(&path, lang_tag) {
                return get_langauge_page_path(&path, lang_tag);
            }
            return path;
        }
        None => return path,
    }
}

pub fn fetch_page(path: &String, lang_tag: Option<&String>) -> Option<Page> {
    match markdown_to_page(path_to_content_path(path, lang_tag, None)) {
        Ok(page) => Some(page),
        Err(e) => {
            warn!("failed to fetch page: {}", e);
            None
        }
    }
}

pub fn fetch_pages(sub_path: Option<String>, lang_tag: Option<&String>) -> Vec<Page> {
    match get_pages(sub_path, lang_tag) {
        Ok(pages) => pages,
        Err(e) => {
            warn!("failed to fetch pages: {}", e);
            Vec::new()
        }
    }
}
