#[macro_use]
extern crate log;

use anyhow::{anyhow, Result};
use futures::future::join_all;
use headers::authorization::Basic;
use headers::{Authorization, HeaderMap, HeaderMapExt, HeaderName};
use indicatif::{ProgressBar, ProgressStyle};
use regex::RegexSet;
use std::{collections::HashSet, convert::TryInto, time::Duration};
use structopt::StructOpt;

mod checker;
mod collector;
mod extract;
mod options;
mod types;

use checker::Checker;
use extract::Uri;
use options::{Config, KimchiOptions};
use types::{Excludes, Status};

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

fn print_summary(found: &HashSet<Uri>, results: &[Status]) {
    let found = found.len();
    let excluded: usize = results
        .iter()
        .filter(|l| matches!(l, Status::Excluded))
        .count();
    let success: usize = results
        .iter()
        .filter(|l| matches!(l, Status::Ok(_)))
        .count();
    let errors: usize = found - excluded - success;

    println!();
    println!("📝Summary");
    println!("-------------------");
    println!("🔍Found: {}", found);
    println!("👻Excluded: {}", excluded);
    println!("✅Successful: {}", success);
    println!("🚫Errors: {}", errors);
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let opts = KimchiOptions::from_args();

    // Load a pontentially existing config file and merge it into the config from the CLI
    let cfg = if let Some(c) = Config::load_from_file(&opts.config_file)? {
        opts.config.merge(c)
    } else {
        opts.config
    };

    let mut runtime = match cfg.threads {
        Some(threads) => {
            // We define our own runtime instead of the `tokio::main` attribute since we want to make the number of threads configurable
            tokio::runtime::Builder::new()
                .threaded_scheduler()
                .core_threads(threads)
                .enable_all()
                .build()?
        }
        None => tokio::runtime::Runtime::new()?,
    };
    let errorcode = runtime.block_on(run(cfg, opts.inputs))?;
    std::process::exit(errorcode);
}

async fn run(cfg: Config, inputs: Vec<String>) -> Result<i32> {
    let includes = RegexSet::new(&cfg.include).ok();
    let excludes = Excludes::from_options(&cfg);
    let mut headers = parse_headers(cfg.headers)?;

    if let Some(auth) = cfg.basic_auth {
        let auth_header = parse_basic_auth(&auth)?;
        headers.typed_insert(auth_header);
    }

    let accepted = match cfg.accept {
        Some(accept) => parse_statuscodes(accept)?,
        None => None,
    };
    let timeout = parse_timeout(cfg.timeout)?;
    let links = collector::collect_links(inputs, cfg.base_url).await?;
    let progress_bar = if cfg.progress {
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
    let checker = Checker::try_new(
        cfg.github_token,
        includes,
        excludes,
        cfg.max_redirects,
        cfg.user_agent,
        cfg.insecure,
        cfg.scheme,
        headers,
        cfg.method.try_into()?,
        accepted,
        Some(timeout),
        cfg.verbose,
        progress_bar.as_ref(),
    )?;

    let futures: Vec<_> = links.iter().map(|l| checker.check(l)).collect();
    let results = join_all(futures).await;

    // note that prints may interfere progress bar so this must go before summary
    if let Some(progress_bar) = progress_bar {
        progress_bar.finish_and_clear();
    }

    if cfg.verbose {
        print_summary(&links, &results);
    }

    let success = results.iter().all(|r| r.is_success() || r.is_excluded());

    match success {
        true => Ok(ExitCode::Success as i32),
        false => Ok(ExitCode::LinkCheckFailure as i32),
    }
}

fn read_header(input: String) -> Result<(String, String)> {
    let elements: Vec<_> = input.split('=').collect();
    if elements.len() != 2 {
        return Err(anyhow!(
            "Header value should be of the form key=value, got {}",
            input
        ));
    }
    Ok((elements[0].into(), elements[1].into()))
}

fn parse_timeout(timeout: String) -> Result<Duration> {
    Ok(Duration::from_secs(timeout.parse::<u64>()?))
}

fn parse_headers(headers: Vec<String>) -> Result<HeaderMap> {
    let mut out = HeaderMap::new();
    for header in headers {
        let (key, val) = read_header(header)?;
        out.insert(
            HeaderName::from_bytes(key.as_bytes())?,
            val.parse().unwrap(),
        );
    }
    Ok(out)
}

fn parse_statuscodes(accept: String) -> Result<Option<HashSet<http::StatusCode>>> {
    let mut statuscodes = HashSet::new();
    for code in accept.split(',').into_iter() {
        let code: reqwest::StatusCode = reqwest::StatusCode::from_bytes(code.as_bytes())?;
        statuscodes.insert(code);
    }
    Ok(Some(statuscodes))
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

#[cfg(test)]
mod test {
    use super::*;
    use http::StatusCode;
    use reqwest::header;

    #[test]
    fn test_parse_custom_headers() {
        let mut custom = HeaderMap::new();
        custom.insert(header::ACCEPT, "text/html".parse().unwrap());
        assert_eq!(
            parse_headers(vec!["accept=text/html".into()]).unwrap(),
            custom
        );
    }

    #[test]
    fn test_parse_statuscodes() {
        let actual = parse_statuscodes("200,204,301".into()).unwrap();
        let expected: Option<HashSet<StatusCode>> = Some(
            [
                StatusCode::OK,
                StatusCode::NO_CONTENT,
                StatusCode::MOVED_PERMANENTLY,
            ]
            .iter()
            .cloned()
            .collect(),
        );
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
