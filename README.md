# Kimchi

![Build/Tests](https://github.com/wgalyen/Kimchi/workflows/Rust/badge.svg)

## Why?

The existing link checkers were not flexible enough for my use-case. Kimchi
runs all requests fully asynchronously and has a low memory/CPU footprint.

kimchi can...

- handle links inside Markdown, HTML, and other documents
- handle chunked encodings
- handle gzip compression
- fake user agents (required for some firewalls)
- skip non-links like anchors or relative URLs
- exclude some websites with regular expressions
- handle a configurable number of redirects
- disguise as a different user agent (like curl)
- optionally ignore SSL certificate errors (`--insecure`)
- check multiple files at once (supports globbing)
- support checking links from any website URL
- limit scheme (e.g. only check HTTPS links with "https")
- accept custom headers (e.g. for cases like https://github.com/rust-lang/crates.io/issues/788)
- show final summary/statistics

SOON:

- automatically retry and backoff
- check relative and absolute URLs
- set timeout for HTTP requests in seconds (`--timeout`). Default is no timeout.
- use `HEAD` requests instead of `GET` to avoid network I/O
- show the progress interactively (`--progress`)

## How?

Set an environment variable with your token like so `GITHUB_TOKEN=xxxx`.

Run it inside a repository with a `README.md` or specify a different Markdown
file with

```
kimchi <yourfile>
```

## Comparison

Collecting other link checkers here to crush them in comparison. :)

- https://github.com/dkhamsing/awesome_bot