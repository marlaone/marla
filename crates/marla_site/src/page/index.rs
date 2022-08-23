use std::{path::PathBuf, sync::Arc};

use ::futures::executor::block_on;
use anyhow::Result;
use glob::glob;
use marla_core::config::site_content_path;
use notify::{event::ModifyKind, Config, EventKind, RecommendedWatcher, Watcher};
use path_clean::PathClean;
use tokio::sync::{mpsc::channel, RwLock};

use super::{markdown::markdown_to_page, Page};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

    pub fn page_by_content_path(&self, content_path: &PathBuf) -> Option<&Page> {
        let cleaned_path = content_path.clean();
        return self.pages.iter().find(|p| p.content_path == cleaned_path);
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

    pub fn watch(page_index: Arc<RwLock<PageIndex>>) {
        let (tx, mut rx) = channel(1);

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                block_on(async {
                    tx.send(res).await.unwrap();
                })
            },
            Config::default(),
        )
        .unwrap();
        tokio::spawn(async move {
            let absolute_site_content_path = std::fs::canonicalize(site_content_path())
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_owned();
            watcher
                .watch(
                    PathBuf::from(&absolute_site_content_path)
                        .as_path()
                        .as_ref(),
                    notify::RecursiveMode::Recursive,
                )
                .unwrap();

            loop {
                match rx.recv().await.unwrap() {
                    Ok(event) => match event.kind {
                        EventKind::Remove(_) => {
                            if event.paths.len() > 0 {
                                let content_path = event.paths[0]
                                    .clone()
                                    .to_str()
                                    .unwrap_or_default()
                                    .to_string();
                                let mut content_path = content_path.replace(
                                    &absolute_site_content_path,
                                    site_content_path().as_str(),
                                );

                                if content_path.starts_with("/") {
                                    content_path = ".".to_string() + content_path.as_str();
                                }

                                match markdown_to_page(PathBuf::from(&content_path).clean()) {
                                    Ok(page) => {
                                        let pages = page_index.read().await.pages.clone();
                                        for (idx, current_page) in pages.iter().enumerate() {
                                            if page.path == current_page.path
                                                && page.lang == current_page.lang
                                            {
                                                page_index.write().await.pages.remove(idx);
                                                break;
                                            }
                                        }
                                    }
                                    Err(e) => log::error!("failed to remove page index = {:?}", e),
                                }
                            }
                        }
                        EventKind::Modify(modify) => match modify {
                            ModifyKind::Data(_) => {
                                if event.paths.len() > 0 {
                                    let content_path = event.paths[0]
                                        .clone()
                                        .to_str()
                                        .unwrap_or_default()
                                        .to_string();
                                    let mut content_path = content_path.replace(
                                        &absolute_site_content_path,
                                        site_content_path().as_str(),
                                    );

                                    if content_path.starts_with("/") {
                                        content_path = ".".to_string() + content_path.as_str();
                                    }

                                    match markdown_to_page(PathBuf::from(&content_path).clean()) {
                                        Ok(page) => {
                                            let pages = page_index.read().await.pages.clone();
                                            for (idx, current_page) in pages.iter().enumerate() {
                                                if page.path == current_page.path
                                                    && page.lang == current_page.lang
                                                {
                                                    page_index.write().await.pages[idx] = page;
                                                    break;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            log::error!("failed to update page index = {:?}", e)
                                        }
                                    }
                                }
                            }
                            ModifyKind::Name(_) => {
                                if event.paths.len() > 0 {
                                    let content_path = event.paths[0]
                                        .clone()
                                        .to_str()
                                        .unwrap_or_default()
                                        .to_string();
                                    let mut content_path = content_path.replace(
                                        &absolute_site_content_path,
                                        site_content_path().as_str(),
                                    );

                                    if content_path.starts_with("/") {
                                        content_path = ".".to_string() + content_path.as_str();
                                    }

                                    let content_path = PathBuf::from(content_path).clean();
                                    let pages = page_index.read().await.pages.clone();
                                    for (idx, current_page) in pages.iter().enumerate() {
                                        if content_path == current_page.content_path {
                                            page_index.write().await.pages.remove(idx);
                                            break;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        },
                        EventKind::Create(_) => {
                            if event.paths.len() > 0 {
                                let content_path = event.paths[0]
                                    .clone()
                                    .to_str()
                                    .unwrap_or_default()
                                    .to_string();
                                let mut content_path = content_path.replace(
                                    &absolute_site_content_path,
                                    site_content_path().as_str(),
                                );

                                if content_path.starts_with("/") {
                                    content_path = ".".to_string() + content_path.as_str();
                                }

                                match markdown_to_page(PathBuf::from(&content_path).clean()) {
                                    Ok(page) => page_index.write().await.pages.push(page),
                                    Err(e) => log::error!("failed to append page index = {:?}", e),
                                }
                            }
                        }

                        _ => (),
                    },
                    Err(e) => println!("watch error: {:?}", e),
                };
            }
        });
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
