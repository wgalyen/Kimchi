# Kimchi

![Build/Tests](https://github.com/wgalyen/Kimchi/workflows/Rust/badge.svg)

## Why?

The existing link checkers were not flexible enough for my use-case.
kimchi can...

- run requests concurrently (fully async Rust)
- handle links inside Markdown, HTML, and other documents
- handle chunked encodings
- handle gzip compression
- fake user agents (required for some firewalls)
- skip non-links like anchors or relative URLs
- exclude some websites with regular expressions
- handle a configurable number of redirects
- disguise as a different user agent (like curl)
- optionally ignore SSL certificate errors
- run with a low memory/CPU footprint

SOON:

- automatically retry and backoff
- check relative and absolute paths
- limit schema
- check multiple files
- support input files using wildcards
- set timeout for HTTP requests in seconds. Disabled by default.
- support checking links from any website URL
- accept custom headers (see https://github.com/rust-lang/crates.io/issues/788)
- use `HEAD` requests instead of `GET` to avoid network I/O
- show the progress
- show final status

## How?

Set an environment variable with your token like so `GITHUB_TOKEN=xxxx`.

Run it inside a repository with a `README.md` or specify a different Markdown
file with

```
kimchi <yourfile>
```