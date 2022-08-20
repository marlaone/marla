use std::collections::HashMap;

use serde_json::{from_value, to_value};
use tera::{Result as TeraResult, Tera, Value};

use crate::page::queries::{all_pages::AllPagesPages, page_by_path::PageByPathPage};

fn get_page_from_args(args: &HashMap<String, Value>) -> Option<PageByPathPage> {
    match args.get("page") {
        Some(value) => match from_value::<PageByPathPage>(value.clone()) {
            Ok(v) => return Some(v),
            Err(_) => {}
        },
        None => {}
    };

    match args.get("page") {
        Some(value) => match from_value::<AllPagesPages>(value.clone()) {
            Ok(v) => return Some(PageByPathPage::from(v)),
            Err(_) => {}
        },
        None => {}
    };

    return None;
}

pub fn apply_tera_functions(theme_tera: &mut Tera) {
    theme_tera.register_function(
        "has_meta_title",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match get_page_from_args(args) {
                Some(v) => match v.meta {
                    Some(meta) => match meta.title {
                        Some(_) => Ok(to_value(true)?),
                        None => Ok(to_value(false)?),
                    },
                    None => Ok(to_value(false)?),
                },
                None => Ok(to_value(false)?),
            }
        }),
    );

    theme_tera.register_function(
        "meta_title",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match get_page_from_args(args) {
                Some(v) => Ok(to_value(
                    v.meta.unwrap_or_default().title.unwrap_or_default(),
                )?),
                None => Ok(to_value("")?),
            }
        }),
    );

    theme_tera.register_function(
        "has_meta_description",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match get_page_from_args(args) {
                Some(v) => match v.meta {
                    Some(meta) => match meta.description {
                        Some(_) => Ok(to_value(true)?),
                        None => Ok(to_value(false)?),
                    },
                    None => Ok(to_value(false)?),
                },
                None => Ok(to_value(false)?),
            }
        }),
    );

    theme_tera.register_function(
        "meta_description",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match get_page_from_args(args) {
                Some(v) => Ok(to_value(
                    v.meta.unwrap_or_default().description.unwrap_or_default(),
                )?),
                None => Ok(to_value("")?),
            }
        }),
    );

    theme_tera.register_function(
        "has_meta_keywords",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match get_page_from_args(args) {
                Some(v) => match v.meta {
                    Some(meta) => match meta.keywords {
                        Some(_) => Ok(to_value(true)?),
                        None => Ok(to_value(false)?),
                    },
                    None => Ok(to_value(false)?),
                },
                None => Ok(to_value(false)?),
            }
        }),
    );

    theme_tera.register_function(
        "meta_keywords",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            match get_page_from_args(args) {
                Some(v) => Ok(to_value(
                    v.meta.unwrap_or_default().keywords.unwrap_or_default(),
                )?),
                None => Ok(to_value(Vec::<String>::new())?),
            }
        }),
    );

    theme_tera.register_function(
        "sub_pages",
        Box::new(move |args: &HashMap<String, Value>| -> TeraResult<Value> {
            let page = get_page_from_args(args);

            let pages = match args.get("pages") {
                Some(value) => match from_value::<Vec<AllPagesPages>>(value.clone()) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                },
                None => None,
            };

            if !page.is_some() || !pages.is_some() {
                return Ok(to_value(Vec::<&AllPagesPages>::new())?);
            }

            return Ok(to_value(
                pages
                    .unwrap_or_default()
                    .iter()
                    .filter(|p| {
                        p.path != page.as_ref().unwrap().path
                            && p.path.starts_with(&page.as_ref().unwrap().path)
                    })
                    .collect::<Vec<&AllPagesPages>>(),
            )?);
        }),
    );
}
