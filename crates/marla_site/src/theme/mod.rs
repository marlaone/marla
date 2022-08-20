use std::collections::HashMap;

use anyhow::{Context, Result};
use glob::glob;
use marla_core::config::{site_content_path, site_theme_path};
use tera::{from_value, to_value, Context as TeraContext, Result as TeraResult, Tera, Value};

use crate::{page::queries::page_by_path::PageByPathPage, site::Site, utils::clean_path};

pub fn get_theme_path() -> Result<String> {
    let mut theme_path = site_theme_path();
    if !theme_path.ends_with("/") {
        theme_path.push_str("/");
    }
    return Ok(theme_path);
}

pub fn apply_tera_functions(theme_tera: &mut Tera) {
    theme_tera.register_function(
        "has_meta_title",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match args.get("page") {
                Some(value) => match from_value::<PageByPathPage>(value.clone()) {
                    Ok(v) => match v.meta {
                        Some(meta) => match meta.title {
                            Some(_) => Ok(to_value(true).unwrap()),
                            None => Ok(to_value(false).unwrap()),
                        },
                        None => Ok(to_value(false).unwrap()),
                    },
                    Err(e) => Err(format!("invalid page argument = {}", e.to_string()).into()),
                },
                None => Err("missing page argument".into()),
            }
        }),
    );

    theme_tera.register_function(
        "meta_title",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match args.get("page") {
                Some(value) => match from_value::<PageByPathPage>(value.clone()) {
                    Ok(v) => {
                        Ok(to_value(v.meta.unwrap_or_default().title.unwrap_or_default()).unwrap())
                    }
                    Err(e) => Err(format!("invalid page argument = {}", e.to_string()).into()),
                },
                None => Err("missing page argument".into()),
            }
        }),
    );

    theme_tera.register_function(
        "has_meta_description",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match args.get("page") {
                Some(value) => match from_value::<PageByPathPage>(value.clone()) {
                    Ok(v) => match v.meta {
                        Some(meta) => match meta.description {
                            Some(_) => Ok(to_value(true).unwrap()),
                            None => Ok(to_value(false).unwrap()),
                        },
                        None => Ok(to_value(false).unwrap()),
                    },
                    Err(e) => Err(format!("invalid page argument = {}", e.to_string()).into()),
                },
                None => Err("missing page argument".into()),
            }
        }),
    );

    theme_tera.register_function(
        "meta_description",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match args.get("page") {
                Some(value) => match from_value::<PageByPathPage>(value.clone()) {
                    Ok(v) => Ok(to_value(
                        v.meta.unwrap_or_default().description.unwrap_or_default(),
                    )
                    .unwrap()),
                    Err(e) => Err(format!("invalid page argument = {}", e.to_string()).into()),
                },
                None => Err("missing page argument".into()),
            }
        }),
    );

    theme_tera.register_function(
        "has_meta_keywords",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match args.get("page") {
                Some(value) => match from_value::<PageByPathPage>(value.clone()) {
                    Ok(v) => match v.meta {
                        Some(meta) => match meta.keywords {
                            Some(_) => Ok(to_value(true).unwrap()),
                            None => Ok(to_value(false).unwrap()),
                        },
                        None => Ok(to_value(false).unwrap()),
                    },
                    Err(e) => Err(format!("invalid page argument = {}", e.to_string()).into()),
                },
                None => Err("missing page argument".into()),
            }
        }),
    );

    theme_tera.register_function(
        "meta_keywords",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match args.get("page") {
                Some(value) => match from_value::<PageByPathPage>(value.clone()) {
                    Ok(v) => Ok(
                        to_value(v.meta.unwrap_or_default().keywords.unwrap_or_default()).unwrap(),
                    ),
                    Err(e) => Err(format!("invalid page argument = {}", e.to_string()).into()),
                },
                None => Err("missing page argument".into()),
            }
        }),
    );
}

pub fn get_theme_templates() -> Result<Tera> {
    let mut theme_path = get_theme_path()?;
    theme_path.push_str("/templates/");
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

    apply_tera_functions(&mut theme_tera);

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

    pub fn render_template(&mut self, template: &str, site: &Site) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", site);

        Ok(self.tera.render(template, &context)?)
    }

    pub fn render_page(&mut self, site: &Site) -> Result<String> {
        let mut context = TeraContext::new();
        context.insert("site", site);

        Ok(self.tera.render("page.html", &context)?)
    }
}
