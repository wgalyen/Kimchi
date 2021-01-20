# Kimchi

![Build/Tests](https://github.com/wgalyen/Kimchi/workflows/Kimchi%20Tests/badge.svg)
![Lints](https://github.com/wgalyen/Kimchi/workflows/Kimchi%20Lints/badge.svg)

TODO: Add screenshots here

## Why?

The existing link checkers were not flexible enough for my use-case. Kimchi
runs all requests fully asynchronously and has a low memory/CPU footprint.

## Features

|                                | kimchi | awesome_bot | muffet | broken-link-checker | linkinator |
| ------------------------------ | ------ | ----------- | ------ | ------------------- | ---------- |
| Language                       | Rust   | Ruby        | Go     | JS                  | TypeScript |
| Static binary                  | ✔️     | ✖️          | ✔️     | ✖️                  | ✖️         |
| Async/Parallel                 | ✔️     | ✔️          | ✔️     | ✔️                  | ✔️         |
| Markdown support               | ✔️     | ✔️          | ✖️     | ✖️                  | ✖️         |
| HTML support                   | ✔️     | ✖️          | ✖️     | ✔️                  | ✔️         |
| Plaintext support              | ✔️     | ✖️          | ✖️     | ✖️                  | ✖️         |
| Website support                | ✔️     | ✖️          | ✔️     | ✔️                  | ✔️         |
| Chunked encodings              | ✔️     | ?           | ?      | ?                   | ?          |
| GZIP compression               | ✔️     | ?           | ?      | ✔️                  | ?          |
| Basic Auth                     | ✖️     | ✖️          | ✖️     | ✔️                  | ✖️         |
| Custom user agent              | ✔️     | ✖️          | ✖️     | ✔️                  | ✖️         |
| Relative URLs                  | ✖️     | ✔️          | ✖️     | ✔️                  | ✔️         |
| Skip relative URLs             | ✔️     | ✖️          | ✖️     | ?                   | ✖️         |
| Include patterns               | ✖️     | ✔️          | ✖️     | ✔️                  | ✖️         |
| Exclude patterns               | ✔️     | ✖️          | ✔️     | ✔️                  | ✔️         |
| Handle redirects               | ✔️     | ✔️          | ✔️     | ✔️                  | ✔️         |
| Ignore SSL                     | ✔️     | ✔️          | ✔️     | ✖️                  | ✖️         |
| File globbing                  | ✔️     | ✔️          | ✖️     | ✖️                  | ✔️         |
| Limit scheme (e.g. only HTTPS) | ✔️     | ✖️          | ✖️     | ✔️                  | ✖️         |
| [Custom headers]               | ✔️     | ✖️          | ✔️     | ✖️                  | ✖️         |
| Summary                        | ✔️     | ✔️          | ✔️     | ?                   | ✔️         |
| `HEAD` requests                | ✔️     | ✔️          | ✖️     | ✔️                  | ✔️         |
| Colored output                 | ✔️     | ?           | ✔️     | ?                   | ✔️         |
| [Filter on status code]        | ✔️     | ✔️          | ✖️     | ✖️                  | ✖️         |
| Custom request timeout         | ✔️     | ✔️          | ✔️     | ✖️                  | ✔️         |
| E-mail links                   | ✔️     | ✖️          | ✖️     | ✖️                  | ✖️         |
| Progress bar                   | ✔️     | ✔️          | ✖️     | ✖️                  | ✖️         |
| Retry and backoff              | ✔️     | ✖️          | ✖️     | ✖️                  | ✔️         |
| Exclude private domains        | ✔️     | ✖️          | ✖️     | ✖️                  | ✖️         |
| [Usable as a library]          | ✖️     | ✔️          | ✖️     | ✔️                  | ✔️         |
| Silent mode                    | ✔️     | ✖️          | ✖️     | ✖️                  | ✔️         |

## Planned features:

- kimchi.toml
- report output in HTML, SQL, CSV, XML, JSON, YAML... format
- report extended statistics: request latency
- recursion
- skip duplicate urls

## How?

Set an environment variable with your token like so `GITHUB_TOKEN=xxxx`.

Run it inside a repository with a `README.md` or specify a file with

```
kimchi <yourfile>
```

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