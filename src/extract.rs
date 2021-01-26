use crate::collector::InputContent;
use crate::uri::Uri;
use html5ever::parse_document;
use html5ever::tendril::{StrTendril, TendrilSink};
use linkify::LinkFinder;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use pulldown_cmark::{Event as MDEvent, Parser, Tag};
use std::collections::HashSet;
use std::path::Path;
use url::Url;

#[derive(Clone, Debug)]
pub enum FileType {
    HTML,
    Markdown,
    Plaintext,
}

impl Default for FileType {
    fn default() -> Self {
        Self::Plaintext
    }
}

impl<P: AsRef<Path>> From<P> for FileType {
    /// Detect if the given path points to a Markdown, HTML, or plaintext file.
    fn from(p: P) -> FileType {
        let path = p.as_ref();
        match path.extension() {
            Some(ext) => match ext {
                _ if ext == "md" => FileType::Markdown,
                _ if (ext == "htm" || ext == "html") => FileType::HTML,
                _ => FileType::Plaintext,
            },
            None => FileType::Plaintext,
        }
    }
}

// Use LinkFinder here to offload the actual link searching in plaintext.
fn find_links(input: &str) -> Vec<linkify::Link> {
    let finder = LinkFinder::new();
    finder.links(input).collect()
}

// Extract unparsed URL strings from a markdown string
fn extract_links_from_markdown(input: &str) -> Vec<String> {
    let parser = Parser::new(input);
    parser
        .flat_map(|event| match event {
            MDEvent::Start(tag) => match tag {
                Tag::Link(_, url, _) | Tag::Image(_, url, _) => vec![url.to_string()],
                _ => vec![],
            },
            MDEvent::Text(txt) => extract_links_from_plaintext(&txt.to_string()),
            MDEvent::Html(html) => extract_links_from_html(&html.to_string()),
            _ => vec![],
        })
        .collect()
}

// Extract unparsed URL strings from a HTML string
fn extract_links_from_html(input: &str) -> Vec<String> {
    let tendril = StrTendril::from(input);
    let rc_dom = parse_document(RcDom::default(), Default::default()).one(tendril);

    let mut urls = Vec::new();

    // we pass mutable urls reference to avoid extra allocations in each
    // recursive descent
    walk_html_links(&mut urls, &rc_dom.document);

    urls
}

/// Recursively walk links in a HTML document, aggregating URL strings in `urls`.
fn walk_html_links(mut urls: &mut Vec<String>, node: &Handle) {
    match node.data {
        NodeData::Text { ref contents } => {
            // escape_default turns tab characters into "\t", newlines into "\n", etc.
            let esc_contents = contents.borrow().escape_default().to_string();
            for link in extract_links_from_plaintext(&esc_contents) {
                urls.push(link);
            }
        }

        NodeData::Comment { ref contents } => {
            for link in extract_links_from_plaintext(&contents.escape_default().to_string()) {
                urls.push(link);
            }
        }

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            for attr in attrs.borrow().iter() {
                let attr_value = attr.value.escape_default().to_string();

                if elem_attr_is_link(attr.name.local.as_ref(), name.local.as_ref()) {
                    urls.push(attr_value);
                } else {
                    for link in extract_links_from_plaintext(&attr_value) {
                        urls.push(link);
                    }
                }
            }
        }

        _ => {}
    }

    // recursively traverse the document's nodes -- this doesn't need any extra
    // exit conditions because the document is a tree
    for child in node.children.borrow().iter() {
        walk_html_links(&mut urls, child);
    }
}

/// Determine if element's attribute contains a link / URL.
fn elem_attr_is_link(attr_name: &str, elem_name: &str) -> bool {
    // See a comprehensive list of attributes that might contain URLs/URIs
    // over at: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes
    matches!(
        (attr_name, elem_name),
        ("href", _)
            | ("src", _)
            | ("srcset", _)
            | ("cite", _)
            | ("data", "object")
            | ("onhashchange", "body")
    )
}

// Extract unparsed URL strings from a plaintext
fn extract_links_from_plaintext(input: &str) -> Vec<String> {
    find_links(input)
        .iter()
        .map(|l| String::from(l.as_str()))
        .collect()
}

pub(crate) fn extract_links(input_content: &InputContent, base_url: Option<Url>) -> HashSet<Uri> {
    let links = match input_content.file_type {
        FileType::Markdown => extract_links_from_markdown(&input_content.content),
        FileType::HTML => extract_links_from_html(&input_content.content),
        FileType::Plaintext => extract_links_from_plaintext(&input_content.content),
    };

    // Only keep legit URLs. This sorts out things like anchors.
    // Silently ignore the parse failures for now.
    let mut uris = HashSet::new();
    for link in links {
        match Url::parse(&link) {
            Ok(url) => {
                uris.insert(Uri::Website(url));
            }
            Err(_) => {
                if link.contains('@') {
                    uris.insert(Uri::Mail(link));
                } else if !Path::new(&link).exists() {
                    if let Some(base_url) = &base_url {
                        if let Ok(new_url) = base_url.join(&link) {
                            uris.insert(Uri::Website(new_url));
                        }
                    }
                }
            }
        };
    }
    uris
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Read};

    fn load_fixture(filename: &str) -> String {
        let fixture_path = Path::new(module_path!())
            .parent()
            .unwrap()
            .join("fixtures")
            .join(filename);

        let file = File::open(fixture_path).expect("Unable to open fixture file");
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();

        buf_reader
            .read_to_string(&mut content)
            .expect("Unable to read fixture file contents");

        content
    }

    #[test]
    fn test_extract_markdown_links() {
        let input = "This is [a test](https://mechanikadesign.com). This is a relative link test [Relative Link Test](relative_link)";
        let links = extract_links(
            &InputContent::from_string(input, FileType::Markdown),
            Some(Url::parse("https://github.com/wgalyen/kimchi/").unwrap()),
        );
        assert_eq!(
            links,
            [
                Uri::Website(Url::parse("https://mechanikadesign.com").unwrap()),
                Uri::Website(
                    Url::parse("https://github.com/wgalyen/kimchi/relative_link").unwrap()
                )
            ]
            .iter()
            .cloned()
            .collect()
        )
    }

    #[test]
    fn test_extract_html_links() {
        let input = r#"<html>
                <div class="row">
                    <a href="https://github.com/wgalyen/kimchi/">
                    <a href="blob/master/README.md">README</a>
                </div>
            </html>"#;

        let links = extract_links(
            &InputContent::from_string(input, FileType::HTML),
            Some(Url::parse("https://github.com/wgalyen/").unwrap()),
        );

        assert_eq!(
            links
                .get(&Uri::Website(
                    Url::parse("https://github.com/wgalyen/blob/master/README.md").unwrap()
                ))
                .is_some(),
            true
        );
    }

    #[test]
    fn test_skip_markdown_anchors() {
        let input = "This is [a test](#lol).";
        let links = extract_links(&InputContent::from_string(input, FileType::Markdown), None);
        assert_eq!(links, HashSet::new())
    }

    #[test]
    fn test_skip_markdown_internal_urls() {
        let input = "This is [a test](./internal).";
        let links = extract_links(&InputContent::from_string(input, FileType::Markdown), None);
        assert_eq!(links, HashSet::new())
    }

    #[test]
    fn test_non_markdown_links() {
        let input =
            "https://mechanikadesign.com and https://mechanikadesign.com/foo/bar?lol=1 at test@example.com";
        let links = extract_links(&InputContent::from_string(input, FileType::Plaintext), None);
        let expected = [
            Uri::Website(Url::parse("https://mechanikadesign.com").unwrap()),
            Uri::Website(Url::parse("https://mechanikadesign.com/foo/bar?lol=1").unwrap()),
            Uri::Mail("test@example.com".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(links, expected)
    }

    #[test]
    #[ignore]
    // TODO: Does this escaping need to work properly?
    // See https://github.com/tcort/markdown-link-check/issues/37
    fn test_md_escape() {
        let input = r#"http://msdn.microsoft.com/library/ie/ms535874\(v=vs.85\).aspx"#;
        let links = find_links(input);
        let expected = "http://msdn.microsoft.com/library/ie/ms535874(v=vs.85).aspx)";
        assert!(links.len() == 1);
        assert_eq!(links[0].as_str(), expected);
    }

    #[test]
    fn test_extract_html5_not_valid_xml() {
        let input = load_fixture("TEST_HTML5.html");
        let links = extract_links(&InputContent::from_string(&input, FileType::HTML), None);

        let expected_links = [
            Uri::Website(Url::parse("https://example.com/head/home").unwrap()),
            Uri::Website(Url::parse("https://example.com/css/style_full_url.css").unwrap()),
            // the body links wouldn't be present if the file was parsed strictly as XML
            Uri::Website(Url::parse("https://example.com/body/a").unwrap()),
            Uri::Website(Url::parse("https://example.com/body/div_empty_a").unwrap()),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(links, expected_links);
    }

    #[test]
    fn test_extract_html5_not_valid_xml_relative_links() {
        let input = load_fixture("TEST_HTML5.html");
        let links = extract_links(
            &InputContent::from_string(&input, FileType::HTML),
            Some(Url::parse("https://example.com").unwrap()),
        );

        let expected_links = [
            Uri::Website(Url::parse("https://example.com/head/home").unwrap()),
            Uri::Website(Url::parse("https://example.com/images/icon.png").unwrap()),
            Uri::Website(Url::parse("https://example.com/css/style_relative_url.css").unwrap()),
            Uri::Website(Url::parse("https://example.com/css/style_full_url.css").unwrap()),
            Uri::Website(Url::parse("https://example.com/js/script.js").unwrap()),
            // the body links wouldn't be present if the file was parsed strictly as XML
            Uri::Website(Url::parse("https://example.com/body/a").unwrap()),
            Uri::Website(Url::parse("https://example.com/body/div_empty_a").unwrap()),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(links, expected_links);
    }

    #[test]
    fn test_extract_html5_lowercase_doctype() {
        // this has been problematic with previous XML based parser
        let input = load_fixture("TEST_HTML5_LOWERCASE_DOCTYPE.html");
        let links = extract_links(&InputContent::from_string(&input, FileType::HTML), None);

        let expected_links = [Uri::Website(
            Url::parse("https://example.com/body/a").unwrap(),
        )]
        .iter()
        .cloned()
        .collect();

        assert_eq!(links, expected_links);
    }

    #[test]
    fn test_extract_html5_minified() {
        // minified HTML with some quirky elements such as href attribute values specified without quotes
        let input = load_fixture("TEST_HTML5_MINIFIED.html");
        let links = extract_links(&InputContent::from_string(&input, FileType::HTML), None);

        let expected_links = [
            Uri::Website(Url::parse("https://example.com/").unwrap()),
            Uri::Website(Url::parse("https://example.com/favicon.ico").unwrap()),
            Uri::Website(Url::parse("https://fonts.externalsite.com").unwrap()),
            Uri::Website(Url::parse("https://example.com/docs/").unwrap()),
            Uri::Website(Url::parse("https://example.com/forum").unwrap()),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(links, expected_links);
    }

    #[test]
    fn test_extract_html5_malformed() {
        // malformed links shouldn't stop the parser from further parsing
        let input = load_fixture("TEST_HTML5_MALFORMED_LINKS.html");
        let links = extract_links(&InputContent::from_string(&input, FileType::HTML), None);

        let expected_links = [Uri::Website(
            Url::parse("https://example.com/valid").unwrap(),
        )]
        .iter()
        .cloned()
        .collect();

        assert_eq!(links, expected_links);
    }

    #[test]
    fn test_extract_html5_custom_elements() {
        // the element name shouldn't matter for attributes like href, src, cite etc
        let input = load_fixture("TEST_HTML5_CUSTOM_ELEMENTS.html");
        let links = extract_links(&InputContent::from_string(&input, FileType::HTML), None);

        let expected_links = [
            Uri::Website(Url::parse("https://example.com/some-weird-element").unwrap()),
            Uri::Website(Url::parse("https://example.com/even-weirder-src").unwrap()),
            Uri::Website(Url::parse("https://example.com/even-weirder-href").unwrap()),
            Uri::Website(Url::parse("https://example.com/citations").unwrap()),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(links, expected_links);
    }
}
