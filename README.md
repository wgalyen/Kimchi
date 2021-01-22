# Kimchi

![Build/Tests](https://github.com/wgalyen/Kimchi/workflows/Kimchi%20Tests/badge.svg)
![Lints](https://github.com/wgalyen/Kimchi/workflows/Kimchi%20Lints/badge.svg)

A fast, async, resource-friendly link checker written in Rust.
For GitHub links, it can optionally use a `GITHUB_TOKEN` to avoid getting blocked by the rate
limiter.

TODO: Add screenshots here

## Features

This comparison is made on a best-effort basis. Please create a PR to fix outdated information.

|                      | kimchi  | [awesome_bot] | [muffet]   | [broken-link-checker] | [linkinator] | [linkchecker] | [markdown-link-check] | [fink]   |
| -------------------- | ------- | ----------- | -------- | ------------------- | ---------- | ----------- | ------------------- | ------ |
| Language             | Rust    | Ruby        | Go       | JS                  | TypeScript | Python      | JS                  | PHP    |
| Async/Parallel       | ![yes]  | ![yes]      | ![yes]   | ![yes]              | ![yes]     | ![yes]      | ![yes]              | ![yes] |
| Static binary        | ![yes]  | ![no]       | ![yes]   | ![no]               | ![no]      | ️![no]       | ![no]               | ![no]  |
| Markdown files       | ![yes]  | ![yes]      | ![no]    | ![no]               | ![no]      | ![yes]      | ️![yes]              | ![no]  |
| HTML files           | ![yes]  | ![no]       | ![no]    | ![yes]              | ![yes]     | ![yes]      | ![no]               | ![no]  |
| Text files           | ![yes]  | ![no]       | ![no]    | ![no]               | ![no]      | ![no]       | ![no]               | ![no]  |
| Website support      | ![yes]  | ![no]       | ![yes]   | ![yes]              | ![yes]     | ![yes]      | ![no]               | ![yes] |
| Chunked encodings    | ![yes]  | ![maybe]    | ![maybe] | ![maybe]            | ![maybe]   | ![no]       | ![yes]              | ![yes] |
| GZIP compression     | ![yes]  | ![maybe]    | ![maybe] | ![yes]              | ![maybe]   | ![yes]      | ![maybe]            | ![no]  |
| Basic Auth           | ![yes]  | ![no]       | ![no]    | ![yes]              | ![no]      | ![yes]      | ![no]               | ![no]  |
| Custom user agent    | ![yes]  | ![no]       | ![no]    | ![yes]              | ![no]      | ![yes]      | ![no]               | ![no]  |
| Relative URLs        | ![yes]  | ![yes]      | ![no]    | ![yes]              | ![yes]     | ![yes]      | ![yes]              | ![yes] |
| Skip relative URLs   | ![yes]  | ![no]       | ![no]    | ![maybe]            | ![no]      | ![no]       | ![no]               | ![no]  |
| Include patterns     | ![yes]️  | ![yes]      | ![no]    | ![yes]              | ![no]      | ![no]       | ![no]               | ![no]  |
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

[awesome_bot]: https://github.com/dkhamsing/awesome_bot
[muffet]: https://github.com/raviqqe/muffet
[broken-link-checker]: https://github.com/stevenvachon/broken-link-checker
[linkinator]: https://github.com/JustinBeckwith/linkinator
[linkchecker]: https://github.com/linkchecker/linkchecker
[markdown-link-check]: https://github.com/tcort/markdown-link-check
[fink]: https://github.com/dantleech/fink
[yes]: ./assets/yes.svg
[no]: ./assets/no.svg
[maybe]: ./assets/maybe.svg
[custom headers]: https://github.com/rust-lang/crates.io/issues/788
[filter status code]: https://github.com/tcort/markdown-link-check/issues/94
[skip private domains]: https://github.com/appscodelabs/liche/blob/a5102b0bf90203b467a4f3b4597d22cd83d94f99/url_checker.go
[use as lib]: https://github.com/raviqqe/liche/issues/13

## Planned features. Please help out!

- Report output in HTML, SQL, CSV, XML, JSON, YAML... format
- Report extended statistics: request latency
- Recursion
- Use colored output (https://crates.io/crates/colored)
- Skip duplicate URLs
- Request throttling

### Installation

### Using pre-built binaries

I provide binaries for Linux, macOS, and Windows for every release. \
You can download them from the [releases page](https://github.com/wgalyen/kimchi/releases).

## Commandline usage

Run it inside a repository with a `README.md`:

```
kimchi
```

You can also specify various types of inputs:

```
# check links on a website:
kimchi https://mechanikadesign.com/

# check links in a remote file:
kimchi https://raw.githubusercontent.com/wgalyen/kimchi/master/README.md

# check links in local file(s):
kimchi README.md
kimchi test.html info.txt

# check links in local files (by shell glob):
kimchi ~/projects/*/README.md

# check links in local files (kimchi supports advanced globbing and ~ expansion):
kimchi "~/projects/big_project/**/README.*"

# ignore case when globbing, displaying progress and check result for each link:
kimchi --glob-ignore-case --progress --verbose "~/projects/**/[r]eadme.*"
```

### GitHub token

Optionally, to avoid getting rate-limited while checking GitHub links, you can
set an environment variable with your Github token like so `GITHUB_TOKEN=xxxx`,
or use the `--github-token` CLI option. It can also be set in the config file.

The token can be generated in your
[GitHub account settings page](https://github.com/settings/tokens). A personal
token with no extra permissions is enough to be able to check public repos links.

### Commandline Parameters

There is an extensive list of commandline parameters to customize the behavior,
see below for a full list.

```
USAGE:
    kimchi [FLAGS] [OPTIONS] [--] [inputs]...

FLAGS:
    -E, --exclude-all-private    Exclude all private IPs from checking. Equivalent to `--exclude-private --exclude-link-
                                 local --exclude-loopback`
        --exclude-link-local     Exclude link-local IP address range from checking
        --exclude-loopback       Exclude loopback IP address range from checking
        --exclude-private        Exclude private IP address ranges from checking
        --glob-ignore-case       Ignore case when expanding filesystem path glob inputs
        --help                   Prints help information
    -i, --insecure               Proceed for server connections considered insecure (invalid TLS)
    -p, --progress               Show progress
        --skip-missing           Skip missing input files (default is to error if they don't exist)
    -V, --version                Prints version information
    -v, --verbose                Verbose program output

OPTIONS:
    -a, --accept <accept>                      Comma-separated list of accepted status codes for valid links
    -b, --base-url <base-url>                  Base URL to check relative URLs
        --basic-auth <basic-auth>              Basic authentication support. E.g. `username:password`
    -c, --config <config-file>                 Configuration file to use [default: ./kimchi.toml]
        --exclude <exclude>...                 Exclude URLs from checking (supports regex)
        --github-token <github-token>          GitHub API token to use when checking github.com links, to avoid rate
                                               limiting [env: GITHUB_TOKEN=]
    -h, --headers <headers>...                 Custom request headers
        --include <include>...                 URLs to check (supports regex). Has preference over all excludes
        --max-concurrency <max-concurrency>    Maximum number of concurrent network requests [default: 128]
    -m, --max-redirects <max-redirects>        Maximum number of allowed redirects [default: 10]
    -X, --method <method>                      Request method [default: get]
    -s, --scheme <scheme>                      Only test links with the given scheme (e.g. https)
    -T, --threads <threads>                    Number of threads to utilize. Defaults to number of cores available to
                                               the system
    -t, --timeout <timeout>                    Website timeout from connect to response finished [default: 20]
    -u, --user-agent <user-agent>              User agent [default: kimchi/0.3.1]

ARGS:
    <inputs>...    The inputs (where to get links to check from). These can be: files (e.g. `README.md`), file globs
                   (e.g. `"~/git/*/README.md"`), remote URLs (e.g. `https://example.com/README.md`) or standard
                   input (`-`) [default: README.md]
```

### Exit codes

- `0` for success (all links checked successfully or excluded/skipped as configured)
- `1` for missing inputs and any unexpected runtime failures or config errors
- `2` for link check failures (if any non-excluded link failed the check)

## Library usage

You can use kimchi as a library for your own projects.
Simply add it as a dependency and build your client:

```rust
use kimchi::ClientBuilder;
use http::StatusCode

let client = ClientBuilder::default().build()?;
let url = Url::parse("https://github.com/wgalyen/kimchi")?;
let response = client.check(Website(url)).await?;
assert!(matches!(response.status, Status::Ok(_)));
```

The client is very customizable, e.g.

```rust
let client = ClientBuilder::default()
    .includes(includes)
    .excludes(excludes)
    .max_redirects(cfg.max_redirects)
    .user_agent(cfg.user_agent)
    .allow_insecure(cfg.insecure)
    .custom_headers(headers)
    .method(method)
    .timeout(timeout)
    .verbose(cfg.verbose)
    .github_token(cfg.github_token)
    .scheme(cfg.scheme)
    .accepted(accepted)
    .build()?;
```


## Troubleshooting and workarounds

I collect a list of common workarounds for various websites in the [troubleshooting guide](./TROUBLESHOOTING.md).

