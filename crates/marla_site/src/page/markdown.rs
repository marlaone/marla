use std::path::PathBuf;

use anyhow::Result;
use chrono::DateTime;
use marla_core::config::SETTINGS;
use marla_markdown::{frontmatter::parse, load_markdown, markdown_to_html};

use super::{meta::PageMeta, Page};

pub fn path_to_content_path(path: String) -> PathBuf {
    let mut content_path = "".to_string();

    let contents_path = PathBuf::from(
        SETTINGS
            .read()
            .unwrap()
            .get_string("site.content")
            .unwrap_or_default(),
    );

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
                content_path.push_str("index.md");
            }
        }
        Err(_) => (),
    }

    if !content_path.ends_with(".md") {
        content_path.push_str(".md");
    }

    content_path = content_path.replace("..", ".");

    return contents_path.as_path().join(content_path);
}

pub fn markdown_to_page(path: PathBuf) -> Result<Page> {
    let file_metadata = std::fs::metadata(&path)?;

    let markdown_content = load_markdown(&path)?;

    let markdown = parse::<PageMeta>(&markdown_content)?;

    let page = Page {
        meta: Some(markdown.meta),
        content: markdown_to_html(&markdown.content_markdown)?,
        last_modified_at: DateTime::from(file_metadata.modified()?),
        created_at: DateTime::from(file_metadata.created()?),
    };

    return Ok(page);
}
