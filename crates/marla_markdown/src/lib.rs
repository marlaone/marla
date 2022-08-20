use std::path::PathBuf;

use ammonia::Builder;
use anyhow::Result;
use maplit::hashset;
use pulldown_cmark::{html, Event, Options, Parser, Tag};

pub mod frontmatter;

pub fn markdown_to_html(markdown_input: &str) -> Result<String> {
    let markdown_options = Options::all();
    let parser = Parser::new_ext(markdown_input, markdown_options).map(|event| match event {
        Event::SoftBreak => Event::HardBreak,
        _ => event,
    });

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

#[must_use]
pub fn strip(markdown: &str) -> String {
    // GFM tables and tasks lists are not enabled.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(&markdown, options);
    let mut tags_stack = Vec::new();
    let mut buffer = String::new();

    // For each event we push into the buffer to produce the plain text version.
    for event in parser {
        match event {
            // The start and end events don't contain the text inside the tag. That's handled by the `Event::Text` arm.
            Event::Start(tag) => {
                start_tag(&tag, &mut buffer, &mut tags_stack);
                tags_stack.push(tag);
            }
            Event::End(tag) => {
                tags_stack.pop();
                end_tag(&tag, &mut buffer, &tags_stack);
            }
            Event::Text(content) => {
                if !tags_stack.iter().any(is_strikethrough) {
                    buffer.push_str(&content)
                }
            }
            Event::Code(content) => buffer.push_str(&content),
            Event::SoftBreak => buffer.push(' '),
            _ => (),
        }
    }
    buffer.trim().to_string()
}

fn start_tag(tag: &Tag, buffer: &mut String, tags_stack: &mut Vec<Tag>) {
    match tag {
        Tag::Link(_, _, title) | Tag::Image(_, _, title) => buffer.push_str(&title),
        Tag::Item => {
            buffer.push('\n');
            let mut lists_stack = tags_stack
                .iter_mut()
                .filter_map(|tag| match tag {
                    Tag::List(nb) => Some(nb),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let prefix_tabs_count = lists_stack.len() - 1;
            for _ in 0..prefix_tabs_count {
                buffer.push('\t')
            }
            if let Some(Some(nb)) = lists_stack.last_mut() {
                buffer.push_str(&nb.to_string());
                buffer.push_str(". ");
                *nb += 1;
            } else {
                buffer.push_str("• ");
            }
        }
        Tag::Paragraph | Tag::CodeBlock(_) | Tag::Heading(_, _, _) => buffer.push('\n'),
        _ => (),
    }
}

fn end_tag(tag: &Tag, buffer: &mut String, tags_stack: &[Tag]) {
    match tag {
        Tag::Paragraph | Tag::Heading(_, _, _) => buffer.push('\n'),
        Tag::CodeBlock(_) => {
            if buffer.chars().last() != Some('\n') {
                buffer.push('\n');
            }
        }
        Tag::List(_) => {
            let is_sublist = tags_stack.iter().any(|tag| match tag {
                Tag::List(_) => true,
                _ => false,
            });
            if !is_sublist {
                buffer.push('\n')
            }
        }
        _ => (),
    }
}

fn is_strikethrough(tag: &Tag) -> bool {
    match tag {
        Tag::Strikethrough => true,
        _ => false,
    }
}
