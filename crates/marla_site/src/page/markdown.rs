use std::path::PathBuf;

use anyhow::Result;
use chrono::DateTime;
use glob::glob;
use log::error;
use marla_core::config::site_content_path;
use marla_markdown::{frontmatter::parse, load_markdown, markdown_to_html, strip};
use voca_rs::Voca;

use crate::utils::clean_path;

use super::{meta::PageMeta, Page};

pub fn path_to_content_path(path: &String, ext: Option<String>) -> PathBuf {
    let content_ext = match ext {
        Some(type_name) => type_name,
        None => ".md".to_string(),
    };

    let mut content_path = "".to_string();

    let contents_path = PathBuf::from(site_content_path());

    if path.starts_with("/") {
        content_path.push_str(".");
        content_path.push_str(path.as_str());
    } else {
        content_path.push_str(path.as_str());
    }

    match std::fs::metadata(contents_path.as_path().join(&content_path)) {
        Ok(content_meta) => {
            if content_meta.is_dir() {
                if !content_path.ends_with("/") {
                    content_path.push_str("/");
                }
                content_path.push_str(("index".to_owned() + content_ext.as_str()).as_str());
            }
        }
        Err(_) => (),
    }

    if !content_path.ends_with(&content_ext) {
        content_path.push_str(&content_ext);
    }

    content_path = content_path.replace("..", ".");

    return clean_path(&contents_path.as_path().join(content_path));
}

pub fn content_path_to_url_path(path: &PathBuf) -> String {
    let contents_path = site_content_path();

    let page_path = String::from(path.to_str().unwrap_or_default())
        .replace(contents_path.as_str(), "")
        .replace(&contents_path.as_str()[1..], "")
        .replace(&contents_path.as_str()[2..], "")
        .replace(".md", "")
        .replace(".html", "")
        .replace("index", "");

    let page_url = PathBuf::from("https://marla.one/")
        .join(page_path)
        .to_str()
        .unwrap_or_default()
        .to_string();

    match url::Url::parse(page_url.as_str()) {
        Ok(url) => {
            let mut page_path = url.path().to_string();

            if page_path != "/" && page_path.ends_with("/") {
                page_path.pop();
            }

            page_path
        }
        Err(_) => "".to_string(),
    }
}

pub fn markdown_to_page(path: PathBuf) -> Result<Page> {
    let file_metadata = std::fs::metadata(&path)?;

    let markdown_content = load_markdown(&path)?;

    let markdown = parse(&markdown_content)?;

    let html = markdown_to_html(&markdown.content_markdown)?;
    let plain = strip(&markdown.content_markdown);

    let page = Page {
        path: content_path_to_url_path(&path),
        meta: Some(PageMeta {
            title: match markdown.params.get("title") {
                Some(title_param) => match title_param.as_str() {
                    Some(title) => Some(title.to_string()),
                    None => None,
                },
                None => None,
            },
            description: match markdown.params.get("description") {
                Some(title_param) => match title_param.as_str() {
                    Some(description) => Some(description.to_string()),
                    None => None,
                },
                None => None,
            },
            keywords: match markdown.params.get("keywords") {
                Some(title_param) => match title_param.as_array() {
                    Some(keywords) => Some(
                        keywords
                            .iter()
                            .map(|k| k.as_str().unwrap_or_default().to_string())
                            .collect(),
                    ),
                    None => None,
                },
                None => None,
            },
        }),
        content: html,
        words: plain._count_words(""),
        plain,
        last_modified_at: DateTime::from(file_metadata.modified()?),
        created_at: DateTime::from(file_metadata.created()?),
        params: markdown.params,
    };

    return Ok(page);
}

pub fn get_pages(sub_path: Option<String>) -> Result<Vec<Page>> {
    let mut pages = Vec::new();

    for &ext in ["md", "html"].iter() {
        let mut contents_path = site_content_path();

        if sub_path.is_some() {
            contents_path.push_str(sub_path.as_ref().unwrap().as_str());
        }

        contents_path.push_str("/**/*.");
        contents_path.push_str(ext);

        for content_entry in glob(&contents_path)? {
            match content_entry {
                Ok(page_path) => pages.push(markdown_to_page(page_path)?),
                Err(e) => error!("{:?}", e),
            }
        }
    }

    pages.sort_by(|a, b| a.path.partial_cmp(&b.path).unwrap());

    Ok(pages)
}
