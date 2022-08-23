use std::path::PathBuf;

use anyhow::Result;
use chrono::DateTime;
use lazy_static::lazy_static;
use marla_core::config::site_content_path;
use marla_markdown::{frontmatter::parse, load_markdown, markdown_to_html, strip};
use regex::Regex;
use voca_rs::Voca;

use crate::{services::page::get_page_path, utils::clean_path};

use super::{meta::PageMeta, Page};

lazy_static! {
    static ref LANG_RE: Regex = Regex::new(r"(\.(?P<l>[a-zA-Z]+))\.(md|html)$").unwrap();
    static ref INDEX_RE: Regex = Regex::new(r"^(?P<p>.*/)?index$").unwrap();
    static ref FILE_PATH_RE: Regex =
        Regex::new(r"(?P<p>[a-zA-Z0-9+_\-/]+)(?:\.([a-zA-Z]+))?\.(md|html)$").unwrap();
}

pub fn path_to_content_path(
    path: &String,
    lang_tag: Option<&String>,
    ext: Option<String>,
) -> PathBuf {
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

    return get_page_path(
        clean_path(&contents_path.as_path().join(&content_path)),
        lang_tag,
    );
}

pub fn content_path_to_url_path(path: &PathBuf) -> String {
    let contents_path = site_content_path();

    let page_path = String::from(path.to_str().unwrap_or_default())
        .replace(contents_path.as_str(), "")
        .replace(&contents_path.as_str()[1..], "")
        .replace(&contents_path.as_str()[2..], "");

    let mut cleaned_page_path = "".to_owned();
    for cap in FILE_PATH_RE.captures_iter(&page_path) {
        cleaned_page_path = cap["p"].to_string();
    }

    let page_path = INDEX_RE
        .replace(&cleaned_page_path, "$p")
        .to_string()
        .replace(".md", "")
        .replace(".html", "");

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

fn get_content_language_by_path(path: &PathBuf) -> Option<String> {
    for caps in LANG_RE.captures_iter(path.to_str().unwrap_or_default()) {
        return Some(caps["l"].to_string());
    }
    return None;
}

pub fn markdown_to_page(path: PathBuf) -> Result<Page> {
    let file_metadata = std::fs::metadata(&path)?;

    let markdown_content = load_markdown(&path)?;

    let markdown = parse(&markdown_content)?;

    let html = markdown_to_html(&markdown.content_markdown)?;
    let plain = strip(&markdown.content_markdown);

    let page = Page {
        path: content_path_to_url_path(&path),
        lang: get_content_language_by_path(&path),
        content_path: path,
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
