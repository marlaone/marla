use anyhow::{Context, Result};
use glob::glob;
use marla_core::config::{site_content_path, site_theme_path};
use tera::{Context as TeraContext, Tera};

use crate::{page::queries::page_by_path::PageByPathPage, utils::clean_path};

pub fn get_theme_path() -> Result<String> {
    let mut theme_path = site_theme_path();
    if !theme_path.ends_with("/") {
        theme_path.push_str("/");
    }
    return Ok(theme_path);
}

pub fn get_theme_templates() -> Result<Tera> {
    let mut theme_path = get_theme_path()?;
    theme_path.push_str("**/*.html");

    let mut theme_tera = Tera::new(std::env::current_dir()?.join(theme_path).to_str().unwrap())
        .with_context(|| format!("failed to parse theme templates"))?;

    let mut contents_path = site_content_path();
    contents_path.push_str("/**/*.html");

    for content_entry in glob(&contents_path)? {
        match content_entry {
            Ok(content_template) => {
                theme_tera.add_template_file(clean_path(&content_template), None)?;
            }
            Err(_) => {}
        };
    }

    return Ok(theme_tera);
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub tera: Tera,
}

impl Theme {
    pub fn new() -> Result<Theme> {
        Ok(Theme {
            tera: get_theme_templates()?,
        })
    }

    pub fn render_template(&mut self, template: &str) -> Result<String> {
        let context = TeraContext::new();

        Ok(self.tera.render(template, &context)?)
    }

    pub fn render_page(&mut self, page: PageByPathPage) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("page", &page);

        Ok(self.tera.render("page.html", &context)?)
    }
}
