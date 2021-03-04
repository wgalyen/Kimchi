use anyhow::{anyhow, Context, Result};
use headers::authorization::Basic;
use headers::{Authorization, HeaderMap, HeaderMapExt, HeaderName};
use indicatif::{ProgressBar, ProgressStyle};
use options::Format;
use regex::RegexSet;
use std::{collections::HashSet, time::Duration};
use std::{fs, str::FromStr};
use structopt::StructOpt;
use tokio::sync::mpsc;

mod options;
mod stats;

use crate::options::{Config, KimchiOptions};
use crate::stats::ResponseStats;

use kimchi::collector::{self, Input};
use kimchi::{ClientBuilder, ClientPool, Response, Status};

/// A C-like enum that can be cast to `i32` and used as process exit code.
enum ExitCode {
    Success = 0,
    // NOTE: exit code 1 is used for any `Result::Err` bubbled up to `main()` using the `?` operator.
    // For now, 1 acts as a catch-all for everything non-link related (including config errors),
    // until we find a way to structure the error code handling better.
    #[allow(unused)]
    UnexpectedFailure = 1,
    LinkCheckFailure = 2,
}

fn main() -> Result<()> {
    // std::process::exit doesn't guarantee that all destructors will be ran,
    // therefore we wrap "main" code in another function to guarantee that.
    // See: https://doc.rust-lang.org/stable/std/process/fn.exit.html
    // Also see: https://www.youtube.com/watch?v=zQC8T71Y8e4
    let exit_code = run_main()?;
    std::process::exit(exit_code);
}

fn run_main() -> Result<i32> {
    let mut opts = KimchiOptions::from_args();

    // Load a potentially existing config file and merge it into the config from the CLI
    if let Some(c) = Config::load_from_file(&opts.config_file)? {
        opts.config.merge(c)
    }
    let cfg = &opts.config;

    let runtime = match cfg.threads {
        Some(threads) => {
            // We define our own runtime instead of the `tokio::main` attribute since we want to make the number of threads configurable
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(threads)
                .enable_all()
                .build()?
        }
        None => tokio::runtime::Runtime::new()?,
    };

    runtime.block_on(run(cfg, opts.inputs()))
}

fn show_progress(progress_bar: &Option<ProgressBar>, response: &Response, verbose: bool) {
    let message = status_message(&response, verbose);
    if let Some(pb) = progress_bar {
        pb.inc(1);
        // regular println! interferes with progress bar
        if let Some(message) = message {
            pb.println(message);
        }
    } else if let Some(message) = message {
        println!("{}", message);
    };
}

fn fmt(stats: &ResponseStats, format: &Format) -> Result<String> {
    Ok(match format {
        Format::String => stats.to_string(),
        Format::JSON => serde_json::to_string(&stats)?,
    })
}

async fn run(cfg: &Config, inputs: Vec<Input>) -> Result<i32> {
    let mut headers = parse_headers(&cfg.headers)?;
    if let Some(auth) = &cfg.basic_auth {
        let auth_header = parse_basic_auth(&auth)?;
        headers.typed_insert(auth_header);
    }

    let accepted = cfg.accept.clone().and_then(|a| parse_statuscodes(&a).ok());
    let timeout = parse_timeout(cfg.timeout);
    let max_concurrency = cfg.max_concurrency;
    let method: reqwest::Method = reqwest::Method::from_str(&cfg.method.to_uppercase())?;
    let include = RegexSet::new(&cfg.include)?;
    let exclude = RegexSet::new(&cfg.exclude)?;

    let client = ClientBuilder::default()
        .includes(include)
        .excludes(exclude)
        .exclude_all_private(cfg.exclude_all_private)
        .exclude_private_ips(cfg.exclude_private)
        .exclude_link_local_ips(cfg.exclude_link_local)
        .exclude_loopback_ips(cfg.exclude_loopback)
        .max_redirects(cfg.max_redirects)
        .user_agent(cfg.user_agent.clone())
        .allow_insecure(cfg.insecure)
        .custom_headers(headers)
        .method(method)
        .timeout(timeout)
        .github_token(cfg.github_token.clone())
        .scheme(cfg.scheme.clone())
        .accepted(accepted)
        .build()?;

    let links = collector::collect_links(
        &inputs,
        cfg.base_url.clone(),
        cfg.skip_missing,
        max_concurrency,
    )
    .await?;
    let pb = if cfg.progress {
        Some(
            ProgressBar::new(links.len() as u64)
            .with_style(
                ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {wide_msg}")
                .progress_chars("#>-")
            )
        )
    } else {
        None
    };

    let (send_req, recv_req) = mpsc::channel(max_concurrency);
    let (send_resp, mut recv_resp) = mpsc::channel(max_concurrency);

    let mut stats = ResponseStats::new();

    let bar = pb.clone();
    tokio::spawn(async move {
        for link in links {
            if let Some(pb) = &bar {
                pb.set_message(&link.to_string());
            };
            send_req.send(link).await.unwrap();
        }
    });

    tokio::spawn(async move {
        // Start receiving requests
        let clients: Vec<_> = (0..max_concurrency).map(|_| client.clone()).collect();
        let mut clients = ClientPool::new(send_resp, recv_req, clients);
        clients.listen().await;
    });

    while let Some(response) = recv_resp.recv().await {
        show_progress(&pb, &response, cfg.verbose);
        stats.add(response);
    }

    // Note that print statements may interfere with the progress bar, so this
    // must go before printing the stats
    if let Some(pb) = &pb {
        pb.finish_and_clear();
    }

    if cfg.verbose {
        println!("\n{}", stats);
    }

    if let Some(output) = &cfg.output {
        fs::write(output, fmt(&stats, &cfg.format)?)
            .context("Cannot write status output to file")?;
    }

    match stats.is_success() {
        true => Ok(ExitCode::Success as i32),
        false => Ok(ExitCode::LinkCheckFailure as i32),
    }
}

fn read_header(input: &str) -> Result<(String, String)> {
    let elements: Vec<_> = input.split('=').collect();
    if elements.len() != 2 {
        return Err(anyhow!(
            "Header value should be of the form key=value, got {}",
            input
        ));
    }
    Ok((elements[0].into(), elements[1].into()))
}

fn parse_timeout(timeout: usize) -> Duration {
    Duration::from_secs(timeout as u64)
}

fn parse_headers<T: AsRef<str>>(headers: &[T]) -> Result<HeaderMap> {
    let mut out = HeaderMap::new();
    for header in headers {
        let (key, val) = read_header(header.as_ref())?;
        out.insert(
            HeaderName::from_bytes(key.as_bytes())?,
            val.parse().unwrap(),
        );
    }
    Ok(out)
}

fn parse_statuscodes<T: AsRef<str>>(accept: T) -> Result<HashSet<http::StatusCode>> {
    let mut statuscodes = HashSet::new();
    for code in accept.as_ref().split(',').into_iter() {
        let code: reqwest::StatusCode = reqwest::StatusCode::from_bytes(code.as_bytes())?;
        statuscodes.insert(code);
    }
    Ok(statuscodes)
}

fn parse_basic_auth(auth: &str) -> Result<Authorization<Basic>> {
    let params: Vec<_> = auth.split(':').collect();
    if params.len() != 2 {
        return Err(anyhow!(
            "Basic auth value should be of the form username:password, received {}",
            auth
        ));
    }
    Ok(Authorization::basic(params[0], params[1]))
}

fn status_message(response: &Response, verbose: bool) -> Option<String> {
    match &response.status {
        Status::Ok(code) if verbose => Some(format!("✅ {} [{}]", response.uri, code)),
        Status::Redirected(code) if verbose => Some(format!("🔀️ {} [{}]", response.uri, code)),
        Status::Excluded if verbose => Some(format!("👻 {}", response.uri)),
        Status::Failed(code) => Some(format!("🚫 {} [{}]", response.uri, code)),
        Status::Error(e) => Some(format!("⚡ {} ({})", response.uri, e)),
        Status::Timeout(code) => match code {
            Some(c) => Some(format!("⌛ {} [{}]", response.uri, c)),
            None => Some(format!("⌛ {}", response.uri)),
        },
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use http::StatusCode;
    use reqwest::header;

    #[test]
    fn test_parse_custom_headers() {
        let mut custom = HeaderMap::new();
        custom.insert(header::ACCEPT, "text/html".parse().unwrap());
        assert_eq!(parse_headers(&["accept=text/html"]).unwrap(), custom);
    }

    #[test]
    fn test_parse_statuscodes() {
        let actual = parse_statuscodes("200,204,301").unwrap();
        let expected: HashSet<StatusCode> = [
            StatusCode::OK,
            StatusCode::NO_CONTENT,
            StatusCode::MOVED_PERMANENTLY,
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_basic_auth() {
        let mut expected = HeaderMap::new();
        expected.insert(
            header::AUTHORIZATION,
            "Basic YWxhZGluOmFicmV0ZXNlc2Ftbw==".parse().unwrap(),
        );

        let mut actual = HeaderMap::new();
        let auth_header = parse_basic_auth("aladin:abretesesamo").unwrap();
        actual.typed_insert(auth_header);
        assert_eq!(expected, actual);
    }
}
