use gumdrop::Options;

#[derive(Debug, Options)]
pub(crate) struct KimchiOptions {
    #[options(free, help = "Input files")]
    pub inputs: Vec<String>,

    #[options(help = "show help")]
    pub help: bool,

    #[options(help = "Verbose program output")]
    pub verbose: bool,

    #[options(help = "Maximum number of allowed redirects", default = "10")]
    pub max_redirects: usize,

    #[options(
        help = "Number of threads to utilize (defaults to  number of cores available to the system"
    )]
    pub threads: Option<usize>,

    #[options(help = "User agent", default = "curl/7.71.1")]
    pub user_agent: String,

    #[options(
        help = "Proceed for server connections considered insecure (invalid TLS)",
        default = "false"
    )]
    pub insecure: bool,

    #[options(help = "Only test links with given scheme (e.g. https)")]
    pub scheme: Option<String>,

    // Accumulate all exclusions in a vector
    #[options(help = "Exclude URLs from checking (supports regex)")]
    pub exclude: Vec<String>,
}