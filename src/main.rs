#[macro_use]
extern crate log;

use anyhow::Result;
use glob::glob;
use regex::RegexSet;
use std::env;
use std::{collections::HashSet, fs};

mod checker;
mod extract;

use checker::{CheckStatus, Checker};
use extract::extract_links;
use futures::future::join_all;

use gumdrop::Options;
use reqwest::Url;

#[derive(Debug, Options)]
struct KimchiOptions {
    #[options(free, help = "Input files")]
    inputs: Vec<String>,

    #[options(help = "show help")]
    help: bool,

    #[options(help = "Verbose program output")]
    verbose: bool,

    #[options(help = "Maximum number of allowed redirects", default = "10")]
    max_redirects: usize,

    #[options(
        help = "Number of threads to utilize (defaults to  number of cores available to the system"
    )]
    threads: Option<usize>,

    #[options(help = "User agent", default = "curl/7.71.1")]
    user_agent: String,

    #[options(
        help = "Proceed for server connections considered insecure (invalid TLS)",
        default = "false"
    )]
    insecure: bool,

    #[options(help = "Only test links with given scheme (e.g. https)")]
    scheme: Option<String>,

    // Accumulate all exclusions in a vector
    #[options(help = "Exclude URLs from checking (supports regex)")]
    exclude: Vec<String>,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let opts = KimchiOptions::parse_args_default_or_exit();

    let mut runtime = match opts.threads {
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
    runtime.block_on(run(opts))?;
    Ok(())
}

fn print_summary(found: &HashSet<Url>, results: &Vec<CheckStatus>) {
    let found = found.len();
    let excluded: usize = results
        .iter()
        .filter(|l| matches!(l, CheckStatus::Excluded))
        .count();
    let success: usize = results
        .iter()
        .filter(|l| matches!(l, CheckStatus::OK))
        .count();
    let errors: usize = found - excluded - success;

    println!("");
    println!("📝Summary");
    println!("-------------------");
    println!("🔍Found: {}", found);
    println!("👻Excluded: {}", excluded);
    println!("✅Successful: {}", success);
    println!("🚫Errors: {}", errors);
}

async fn collect_links(inputs: Vec<String>) -> Result<HashSet<Url>> {
    let mut links = HashSet::new();

    for input in inputs {
        match Url::parse(&input) {
            Ok(url) => {
                let res = reqwest::get(url).await?;
                let content = res.text().await?;
                links.extend(extract_links(&content));
            }
            Err(_) => {
                // Assume we received a single file or a glob on our hands
                for entry in glob(&input)? {
                    match entry {
                        Ok(path) => {
                            let content = fs::read_to_string(path)?;
                            links.extend(extract_links(&content));
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }
            }
        };
    }
    Ok(links)
}

async fn run(opts: KimchiOptions) -> Result<()> {
    let excludes = RegexSet::new(opts.exclude).unwrap();

    let checker = Checker::try_new(
        env::var("GITHUB_TOKEN")?,
        Some(excludes),
        opts.max_redirects,
        opts.user_agent,
        opts.insecure,
        opts.scheme,
        opts.verbose,
    )?;

    let links = collect_links(opts.inputs).await?;
    let futures: Vec<_> = links.iter().map(|l| checker.check(&l)).collect();
    let results = join_all(futures).await;

    if opts.verbose {
        print_summary(&links, &results);
    }
    let errorcode = if results.iter().all(|r| r.is_success()) {
        0
    } else {
        1
    };
    std::process::exit(errorcode)
}
