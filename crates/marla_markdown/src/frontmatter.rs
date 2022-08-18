use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Markdown<T> {
    pub meta: T,
    pub content_markdown: String,
}

const FRONTMATTER_DELIMITER: &str = "---";

pub fn parse_frontmatter<'de, T>(post: &'de str) -> Result<(T, &'de str), toml::de::Error>
where
    T: serde::Deserialize<'de> + Default,
{
    if !post.starts_with(FRONTMATTER_DELIMITER) {
        return Ok((T::default(), post));
    }

    let slice = &post[FRONTMATTER_DELIMITER.len()..];
    let index_of_ending_line = slice.find(FRONTMATTER_DELIMITER).unwrap_or(0);
    if index_of_ending_line == 0 {
        return Ok((T::default(), post));
    }

    return Ok((
        toml::from_str(&slice[..index_of_ending_line])?,
        &slice[(index_of_ending_line + FRONTMATTER_DELIMITER.len())..],
    ));
}

pub fn parse<'de, T>(source: &'de str) -> Result<Markdown<T>, toml::de::Error>
where
    T: serde::Deserialize<'de> + Default,
{
    let (meta, rest): (T, &'de str) = parse_frontmatter(source)?;

    Ok(Markdown {
        meta,
        content_markdown: String::from(rest),
    })
}
