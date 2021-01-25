use crate::collector::InputContent;
use crate::uri::Uri;
use linkify::LinkFinder;
use pulldown_cmark::{Event as MDEvent, Parser, Tag};
use quick_xml::{events::Event as HTMLEvent, Reader};
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

// Use LinkFinder here to offload the actual link searching
fn find_links(input: &str) -> Vec<linkify::Link> {
    let finder = LinkFinder::new();
    finder.links(input).collect()
}

// Extracting unparsed URL strings from a markdown string
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

// Extracting unparsed URL strings from a HTML string
fn extract_links_from_html(input: &str) -> Vec<String> {
    let mut reader = Reader::from_str(input);

    // allow not well-formed XML documents, which contain non-closed elements
    // (e.g. HTML5 which has things like `<link>`)
    reader.check_end_names(false);

    let mut buf = Vec::new();
    let mut urls = Vec::new();

    while let Ok(e) = reader.read_event(&mut buf) {
        match e {
            HTMLEvent::Start(ref e) | HTMLEvent::Empty(ref e) => {
                for attr in e.attributes() {
                    if let Ok(attr) = attr {
                        match (attr.key, e.name()) {
                            (b"href", b"a")
                            | (b"href", b"area")
                            | (b"href", b"base")
                            | (b"href", b"link")
                            | (b"src", b"audio")
                            | (b"src", b"embed")
                            | (b"src", b"iframe")
                            | (b"src", b"img")
                            | (b"src", b"input")
                            | (b"src", b"script")
                            | (b"src", b"source")
                            | (b"src", b"track")
                            | (b"src", b"video")
                            | (b"srcset", b"img")
                            | (b"srcset", b"source")
                            | (b"cite", b"blockquote")
                            | (b"cite", b"del")
                            | (b"cite", b"ins")
                            | (b"cite", b"q")
                            | (b"data", b"object")
                            | (b"onhashchange", b"body") => {
                                urls.push(String::from_utf8_lossy(attr.value.as_ref()).to_string());
                            }
                            _ => {
                                for link in extract_links_from_plaintext(
                                    &String::from_utf8_lossy(attr.value.as_ref()).to_string(),
                                ) {
                                    urls.push(link);
                                }
                            }
                        }
                    }
                }
            }
            HTMLEvent::Text(txt) | HTMLEvent::Comment(txt) => {
                for link in extract_links_from_plaintext(
                    &String::from_utf8_lossy(txt.escaped()).to_string(),
                ) {
                    urls.push(link);
                }
            }
            HTMLEvent::Eof => {
                break;
            }
            _ => {}
        }
        buf.clear();
    }
    urls
}

// Extracting unparsed URL strings from a plaintext
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
        let test_html5 = Path::new(module_path!())
            .parent()
            .unwrap()
            .join("fixtures")
            .join("TEST_HTML5.html");

        let file = File::open(test_html5).expect("Unable to open test file");
        let mut buf_reader = BufReader::new(file);
        let mut input = String::new();
        buf_reader
            .read_to_string(&mut input)
            .expect("Unable to read test file contents");

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
        let test_html5 = Path::new(module_path!())
            .parent()
            .unwrap()
            .join("fixtures")
            .join("TEST_HTML5.html");

        let file = File::open(test_html5).expect("Unable to open test file");
        let mut buf_reader = BufReader::new(file);
        let mut input = String::new();
        buf_reader
            .read_to_string(&mut input)
            .expect("Unable to read test file contents");

        let links = extract_links(
            &InputContent::from_string(&input, FileType::HTML),
            Some(Url::parse("https://example.com").unwrap()),
        );
        let expected_links = [
            Uri::Website(Url::parse("https://example.com/head/home").unwrap()),
            Uri::Website(Url::parse("https://example.com/images/icon.png").unwrap()),
            Uri::Website(Url::parse("https://example.com/css/style_relative_url.css").unwrap()),
            Uri::Website(Url::parse("https://example.com/css/style_full_url.css").unwrap()),
            // TODO BUG: the JS link is missing because the parser can't properly deal
            //           with `<script defer src="..."></script>` (tags that have attributes with no value)
            // Uri::Website(Url::parse("https://example.com/js/script.js").unwrap()),

            // the body links wouldn't be present if the file was parsed strictly as XML
            Uri::Website(Url::parse("https://example.com/body/a").unwrap()),
            Uri::Website(Url::parse("https://example.com/body/div_empty_a").unwrap()),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(links, expected_links);
    }
}
