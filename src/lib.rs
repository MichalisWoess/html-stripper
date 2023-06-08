// delete svgs, images, style, script, and comments, head
// extraxt inner html
// <span> --> title

#![allow(unstable_name_collisions)]

use ego_tree::NodeRef;
use itertools::Itertools;
use scraper::{Html, Node};

pub fn strip(html: &str) -> String {
    let html = scraper::Html::parse_document(html);

    if !html.errors.is_empty() {
        eprintln!("[WARNING] Parsing with errors...");
    }

    html.tree
        .nodes()
        .filter(|n| should_process_children(n.parent()))
        .filter_map(|n| n.value().as_text())
        .map(|t| t.text.trim())
        .filter(|t| !t.is_empty())
        .map(remove_extra_whitespaces)
        .intersperse("\n".to_owned())
        .collect::<String>()
        + "\n"
        + &extract_relevant_tag_attribute(&html)
}

const IGNORED_TAGS: [&str; 5] = ["svg", "img", "style", "script", "head"];

fn should_process_children(parent: Option<NodeRef<Node>>) -> bool {
    match parent.map(|p| p.value()) {
        Some(Node::Element(e)) => IGNORED_TAGS.iter().all(|t| e.name() != *t),
        _ => true,
    }
}

fn extract_relevant_tag_attribute(input: &Html) -> String {
    let span_selector =
        scraper::Selector::parse("span").expect("<span> selector could not be parsed");

    let mut result = String::new();

    input
        .select(&span_selector) // select all <span> tags
        .filter_map(|n| n.value().attr("title")) // get title attribute
        .for_each(|title| {
            result.push_str(title);
            result.push('\n');
        });

    result
}

fn remove_extra_whitespaces(input: &str) -> String {
    let result = input.split_whitespace().intersperse(" ").collect();

    result
}

#[cfg(test)]
mod test {
    #[test]
    fn strip_file_test() {
        let html = std::fs::read_to_string("test.html").unwrap();
        let html = super::strip(&html);
        println!("{}", html);
    }
}
