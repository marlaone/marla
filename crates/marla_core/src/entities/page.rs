use std::{collections::BTreeMap, path::PathBuf};

use super::fields::{
    page_draft::PageDraft, page_template_path::PageTemplatePath, required_string::RequiredString,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Page {
    pub title: RequiredString,
    pub path: PathBuf,
    pub content_path: PathBuf,
    pub content: RequiredString,
    pub plain_content: RequiredString,
    pub last_modified_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub words: usize,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub slug: Option<String>,
    pub aliases: Vec<String>,
    pub template_path: PageTemplatePath,
    pub authors: Vec<String>,
    pub draft: PageDraft,
    pub extra: BTreeMap<String, String>,
    pub lang: Option<String>,
}
