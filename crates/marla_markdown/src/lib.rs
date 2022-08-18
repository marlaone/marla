use std::path::PathBuf;

use ammonia::Builder;
use anyhow::Result;
use maplit::hashset;
use pulldown_cmark::{html, Options, Parser};

pub mod frontmatter;

pub fn markdown_to_html(markdown_input: &str) -> Result<String> {
    let markdown_options = Options::all();
    let parser = Parser::new_ext(markdown_input, markdown_options);

    let mut html_output = String::with_capacity(markdown_input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    html_output = Builder::new()
        .generic_attributes(hashset!["id", "class"])
        .clean(&html_output)
        .to_string();

    return Ok(html_output);
}

pub fn load_markdown(path: &PathBuf) -> Result<String> {
    return Ok(std::fs::read_to_string(path)?);
}
