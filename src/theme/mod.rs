use anyhow::Result;
use tera::{Context, Tera};

use crate::{config::SETTINGS, page::queries::page_by_path::PageByPathPage};

pub fn get_theme_path() -> Result<String> {
    let mut theme_path = SETTINGS.read().unwrap().get_string("site.theme")?;
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

        if SETTINGS
            .read()
            .unwrap()
            .get_bool("site.debug")
            .unwrap_or_default()
        {
            self.tera.full_reload()?;
        }

        Ok(self.tera.render("page.html", &context)?)
    }
}
