use anyhow::Result;
use marla_core::config::{site_debug, site_theme_path};
use tera::{Context, Tera};

use crate::page::queries::page_by_path::PageByPathPage;

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

    println!(
        "theme_path = {}",
        std::env::current_dir()?.join(&theme_path).to_str().unwrap()
    );

    return Ok(Tera::new(
        std::env::current_dir()?.join(theme_path).to_str().unwrap(),
    )?);
}

#[derive(Debug, Clone)]
pub struct Theme {
    tera: Tera,
}

impl Theme {
    pub fn new() -> Result<Theme> {
        Ok(Theme {
            tera: get_theme_templates()?,
        })
    }

    pub fn render_page(&mut self, page: PageByPathPage) -> Result<String> {
        let mut context = Context::new();
        context.insert("page", &page);

        if site_debug() {
            self.tera.full_reload()?;
        }

        Ok(self.tera.render("page.html", &context)?)
    }
}
