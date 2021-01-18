# Kimchi
![kimchi](https://cdn.emojidex.com/emoji/seal/kimchi.png "kimchi")

![Rust](https://github.com/wgalyen/kimchi/workflows/Rust/badge.svg)

## Why?

The existing link checkers were not flexible enough for my use-case.
kimchi can...

- run fully asynchronously
- handle links inside unstructured (e.g. non-Markdown) documents
- handle chunked encodings
- handle gzip compression
- fake user agents (required for some firewalls)
- skip non-links like anchors or relative URLs
- exclude some websites with regular expressions
- handle a configurable number of redirects
- SOON: automatically retry and backoff
- SOON: optionally ignore SSL certificate errors

## How?

Set an environment variable with your token like so `GITHUB_TOKEN=xxxx`.

Run it inside a repository with a `README.md` or specify a different Markdown
file with

```
kimchi --input <yourfile.md>
```