# Kimchi
![kimchi](https://cdn.emojidex.com/emoji/seal/kimchi.png "kimchi")

![Rust](https://github.com/wgalyen/kimchi/workflows/Rust/badge.svg)

## Why?

The existing link checkers were not flexible enough for my use-case.
kimchi can...

- Handle chunked encodings
- Handle gzip
- Fake user agents (required for some firewalls)
- Skip non-links like anchors or relative URLs
- SOON: Ignore SSL certificate errors
- SOON: fully async code execution

## How?

Set an environment variable with your token like so `GITHUB_TOKEN=xxxx`.

Run it inside a repository with a `README.md` or specify a different Markdown
file with

```
kimchi --input <yourfile.md>
```