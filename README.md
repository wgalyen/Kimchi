# Kimchi

![Build/Tests](https://github.com/wgalyen/Kimchi/workflows/Kimchi%20Tests/badge.svg)

TODO: Add screenshots here

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
- optionally use `HEAD` requests instead of `GET`
- show colored output
- filter based on status codes (https://github.com/tcort/markdown-link-check/issues/94)
  (e.g. `--accept 200,204`)
- accept a connect timeout (`--connect-timeout`). Default is 20s. Set to 0 for no timeout.

SOON:

- automatically retry and backoff
- set timeout for HTTP requests in seconds (`--timeout`). Default is 10s. Set to 0 for no timeout.
- check relative (`base-url` to set project root)
- show the progress interactively (`--progress`)
- usable as a library (https://github.com/raviqqe/liche/issues/13)
- exclude private domains (https://github.com/appscodelabs/liche/blob/a5102b0bf90203b467a4f3b4597d22cd83d94f99/url_checker.go)
- recursion
- check mailto links (disable with `no-mailto`) (https://www.npmjs.com/package/isemail)

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
- https://github.com/tcort/markdown-link-check
- https://github.com/raviqqe/liche
- https://github.com/raviqqe/muffet