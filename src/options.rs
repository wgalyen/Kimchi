use anyhow::{Error, Result};
use serde::Deserialize;
use std::{fs, io::ErrorKind};
use structopt::StructOpt;

const USER_AGENT: &str = "curl/7.71.1";
const METHOD: &str = "get";
const TIMEOUT: &str = "20";

// Macro for generating default functions to be used by serde
macro_rules! default_function {
    ( $( $name:ident : $T:ty = $e:expr; )* ) => {
        $(
            fn $name() -> $T {
                $e
            }
        )*
    };
}

// Macro for merging configuration values
macro_rules! fold_in {
    ( $cli:ident , $toml:ident ; $( $key:ident : $default:expr; )* ) => {
        $(
            if $cli.$key == $default && $toml.$key != $default {
                $cli.$key = $toml.$key;
            }
        )*
    };
}
#[derive(Debug, StructOpt)]
#[structopt(name = "kimchi", about = "A spicy link checker")]
pub(crate) struct KimchiOptions {
    /// Input files
    pub inputs: Vec<String>,

    /// Configuration file to use
    #[structopt(short, long = "config", default_value = "./kimchi.toml")]
    pub config_file: String,

    #[structopt(flatten)]
    pub config: Config,
}

#[derive(Debug, Deserialize, StructOpt)]
pub(crate) struct Config {
    /// Verbose program output
    #[structopt(short, long)]
    #[serde(default)]
    pub verbose: bool,

    /// Show progress
    #[structopt(short, long)]
    #[serde(default)]
    pub progress: bool,

    /// Maximum number of allowed redirects
    #[structopt(short, long, default_value = "10")]
    #[serde(default)]
    pub max_redirects: usize,

    /// Number of threads to utilize.
    /// Defaults to number of cores available to the system
    #[structopt(short = "T", long)]
    #[serde(default)]
    pub threads: Option<usize>,

    /// User agent
    #[structopt(short, long, default_value = USER_AGENT)]
    #[serde(default = "user_agent")]
    pub user_agent: String,

    /// Proceed for server connections considered insecure (invalid TLS)
    #[structopt(short, long)]
    #[serde(default)]
    pub insecure: bool,

    /// Only test links with the given scheme (e.g. https)
    #[structopt(short, long)]
    #[serde(default)]
    pub scheme: Option<String>,

    /// Exclude URLs from checking (supports regex)
    #[structopt(short, long)]
    #[serde(default)]
    pub exclude: Vec<String>,

    /// Exclude all private IPs from checking.
    /// Equivalent to `--exclude-private --exclude-link-local --exclude-loopback`
    #[structopt(short = "E", long)]
    #[serde(default)]
    pub exclude_all_private: bool,

    /// Exclude private IP address ranges from checking
    #[structopt(long)]
    #[serde(default)]
    pub exclude_private: bool,

    /// Exclude link-local IP address range from checking
    #[structopt(long)]
    #[serde(default)]
    pub exclude_link_local: bool,

    /// Exclude loopback IP address range from checking
    #[structopt(long)]
    #[serde(default)]
    pub exclude_loopback: bool,

    /// Custom request headers
    #[structopt(short, long)]
    #[serde(default)]
    pub headers: Vec<String>,

    /// Comma-separated list of accepted status codes for valid links
    #[structopt(short, long)]
    #[serde(default)]
    pub accept: Option<String>,

    /// Website timeout from connect to response finished
    #[structopt(short, long, default_value = TIMEOUT)]
    #[serde(default = "timeout")]
    pub timeout: String,

    /// Request method
    #[structopt(short = "M", long, default_value = METHOD)]
    #[serde(default = "method")]
    pub method: String,

    #[structopt(short, long, help = "Base URL to check relative URLs")]
    #[serde(default)]
    pub base_url: Option<String>,
}

impl Config {
    /// Load configuration from a file
    pub(crate) fn load_from_file(path: &str) -> Result<Option<Config>> {
        // Read configuration file
        let result = fs::read(path);

        // Ignore a file not found error
        let contents = match result {
            Ok(c) => c,
            Err(e) => {
                return match e.kind() {
                    ErrorKind::NotFound => {
                        println!("[WARN] could not find configuration file, using arguments");
                        Ok(None)
                    }
                    _ => Err(Error::from(e)),
                }
            }
        };

        Ok(Some(toml::from_slice(&contents)?))
    }

    /// Merge the configuration from TOML into the CLI configuration
    pub(crate) fn merge(mut self, toml: Config) -> Config {
        fold_in! {
            // Destination and source configs
            self, toml;

            // Keys with defaults to assign
            verbose: false;
            progress: false;
            max_redirects: 10;
            threads: None;
            user_agent: USER_AGENT;
            insecure: false;
            scheme: None;
            exclude: Vec::<String>::new();
            exclude_all_private: false;
            exclude_private: false;
            exclude_link_local: false;
            exclude_loopback: false;
            headers: Vec::<String>::new();
            accept: None;
            timeout: TIMEOUT;
            method: METHOD;
            base_url: None;
        }

        self
    }
}

// Generate the functions for serde defaults
default_function! {
    user_agent: String = USER_AGENT.to_string();
    timeout: String = TIMEOUT.to_string();
    method: String = METHOD.to_string();
}
