# Kimchi

![Build/Tests](https://github.com/wgalyen/Kimchi/workflows/Kimchi%20Tests/badge.svg)
![Lints](https://github.com/wgalyen/Kimchi/workflows/Kimchi%20Lints/badge.svg)

A fast, async, resource-friendly link checker written in Rust.
For GitHub links, it can optionally use a `GITHUB_TOKEN` to avoid getting blocked by the rate
limiter.

TODO: Add screenshots here

## Features

This comparison is made on a best-effort basis. Please create a PR to fix outdated information.

|                      | kimchi  | awesome_bot | muffet   | broken-link-checker | linkinator | linkchecker | markdown-link-check | fink   |
| -------------------- | ------- | ----------- | -------- | ------------------- | ---------- | ----------- | ------------------- | ------ |
| Language             | Rust    | Ruby        | Go       | JS                  | TypeScript | Python      | JS                  | PHP    |
| Async/Parallel       | ![yes]  | ![yes]      | ![yes]   | ![yes]              | ![yes]     | ![yes]      | ![yes]              | ![yes] |
| Static binary        | ![yes]  | ![no]       | ![yes]   | ![no]               | ![no]      | ️ ![no]     | ![no]               | ![no]  |
| Markdown files       | ![yes]  | ![yes]      | ![no]    | ![no]               | ![no]      | ![no]       | ️ ![yes]            | ![no]  |
| HTML files           | ![yes]  | ![no]       | ![no]    | ![yes]              | ![yes]     | ![no]       | ![no]               | ![no]  |
| Txt files            | ![yes]  | ![no]       | ![no]    | ![no]               | ![no]      | ![no]       | ![no]               | ![no]  |
| Website support      | ![yes]  | ![no]       | ![yes]   | ![yes]              | ![yes]     | ![yes]      | ![no]               | ![yes] |
| Chunked encodings    | ![yes]  | ![maybe]    | ![maybe] | ![maybe]            | ![maybe]   | ![no]       | ![yes]              | ![yes] |
| GZIP compression     | ![yes]  | ![maybe]    | ![maybe] | ![yes]              | ![maybe]   | ![yes]      | ![maybe]            | ![no]  |
| Basic Auth           | ![yes]  | ![no]       | ![no]    | ![yes]              | ![no]      | ![yes]      | ![no]               | ![no]  |
| Custom user agent    | ![yes]  | ![no]       | ![no]    | ![yes]              | ![no]      | ![yes]      | ![no]               | ![no]  |
| Relative URLs        | ![yes]  | ![yes]      | ![no]    | ![yes]              | ![yes]     | ![yes]      | ![yes]              | ![yes] |
| Skip relative URLs   | ![yes]  | ![no]       | ![no]    | ![maybe]            | ![no]      | ![no]       | ![no]               | ![no]  |
| Include patterns     | ![yes]️ | ![yes]      | ![no]    | ![yes]              | ![no]      | ![no]       | ![no]               | ![no]  |
| Exclude patterns     | ![yes]  | ![no]       | ![yes]   | ![yes]              | ![yes]     | ![yes]      | ![yes]              | ![yes] |
| Handle redirects     | ![yes]  | ![yes]      | ![yes]   | ![yes]              | ![yes]     | ![yes]      | ![yes]              | ![yes] |
| Ignore insecure SSL  | ![yes]  | ![yes]      | ![yes]   | ![no]               | ![no]      | ![yes]      | ![no]               | ![yes] |
| File globbing        | ![yes]  | ![yes]      | ![no]    | ![no]               | ![yes]     | ![no]       | ![yes]              | ![no]  |
| Limit scheme         | ![yes]  | ![no]       | ![no]    | ![yes]              | ![no]      | ![yes]      | ![no]               | ![no]  |
| [Custom headers]     | ![yes]  | ![no]       | ![yes]   | ![no]               | ![no]      | ![no]       | ![yes]              | ![yes] |
| Summary              | ![yes]  | ![yes]      | ![yes]   | ![maybe]            | ![yes]     | ![yes]      | ![no]               | ![yes] |
| `HEAD` requests      | ![yes]  | ![yes]      | ![no]    | ![yes]              | ![yes]     | ![yes]      | ![no]               | ![no]  |
| Colored output       | ![yes]  | ![maybe]    | ![yes]   | ![maybe]            | ![yes]     | ![yes]      | ![no]               | ![yes] |
| [Filter status code] | ![yes]  | ![yes]      | ![no]    | ![no]               | ![no]      | ![no]       | ![yes]              | ![no]  |
| Custom timeout       | ![yes]  | ![yes]      | ![yes]   | ![no]               | ![yes]     | ![yes]      | ![no]               | ![yes] |
| E-mail links         | ![yes]  | ![no]       | ![no]    | ![no]               | ![no]      | ![yes]      | ![no]               | ![no]  |
| Progress bar         | ![yes]  | ![yes]      | ![no]    | ![no]               | ![no]      | ![yes]      | ![yes]              | ![yes] |
| Retry and backoff    | ![yes]  | ![no]       | ![no]    | ![no]               | ![yes]     | ![no]       | ![yes]              | ![no]  |
| Skip private domains | ![yes]  | ![no]       | ![no]    | ![no]               | ![no]      | ![no]       | ![no]               | ![no]  |
| [Use as lib]         | ![no]   | ![yes]      | ![no]    | ![yes]              | ![yes]     | ![no]       | ![yes]              | ![no]  |
| Quiet mode           | ![yes]  | ![no]       | ![no]    | ![no]               | ![yes]     | ![yes]      | ![yes]              | ![yes] |
| Config file          | ![yes]  | ![no]       | ![no]    | ![no]               | ![yes]     | ![yes]      | ![yes]              | ![no]  |

[yes]: ./assets/yes.svg
[no]: ./assets/no.svg
[maybe]: ./assets/maybe.svg

## Planned features. Please help out!

- report output in HTML, SQL, CSV, XML, JSON, YAML... format
- report extended statistics: request latency
- recursion
- skip duplicate urls
- request throttling

## How?

Optional (to avoid being rate limited for GitHub links): set an environment variable with your token
like so `GITHUB_TOKEN=xxxx`, or use the `--github-token` CLI option. This can also be set in the
config file.

Run it inside a repository with a `README.md` or specify a file with

```
kimchi <yourfile>
```

### CLI exit codes

- `0` for success (all links checked successfully or excluded/skipped as configured)
- `1` for any unexpected runtime failures or config errors
- `2` for link check failures (if any non-excluded link failed the check)

## Comparison

Collecting other link checkers here to crush them in comparison. :)

- https://github.com/dkhamsing/awesome_bot
- https://github.com/tcort/markdown-link-check
- https://github.com/raviqqe/liche
- https://github.com/raviqqe/muffet
- https://github.com/stevenvachon/broken-link-checker
- https://github.com/JustinBeckwith/linkinator
- https://github.com/linkchecker/linkchecker
- https://github.com/dantleech/fink
- https://github.com/bartdag/pylinkvalidator
- https://github.com/victoriadrake/hydra-link-checker
