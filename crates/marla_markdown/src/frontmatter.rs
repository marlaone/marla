use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Markdown {
    pub params: toml::value::Table,
    pub content_markdown: String,
}

const FRONTMATTER_DELIMITER: &str = "---";

pub fn parse_frontmatter<'de>(
    post: &'de str,
) -> Result<(toml::value::Table, &'de str), toml::de::Error> {
    if !post.starts_with(FRONTMATTER_DELIMITER) {
        return Ok((toml::value::Table::new(), post));
    }

    let slice = &post[FRONTMATTER_DELIMITER.len()..];
    let index_of_ending_line = slice.find(FRONTMATTER_DELIMITER).unwrap_or(0);
    if index_of_ending_line == 0 {
        return Ok((toml::value::Table::new(), post));
    }

    return Ok((
        toml::from_str(&slice[..index_of_ending_line])?,
        &slice[(index_of_ending_line + FRONTMATTER_DELIMITER.len())..],
    ));
}

pub fn parse<'de>(source: &'de str) -> Result<Markdown, toml::de::Error> {
    let (params, rest): (toml::value::Table, &'de str) = parse_frontmatter(source)?;

    Ok(Markdown {
        params,
        content_markdown: String::from(rest),
    })
}
