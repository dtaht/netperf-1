use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ServerOpts {
    /// Run in server mode
    #[structopt(short, long, group = "server_or_client")]
    pub server: bool,
    /// Run in server mode
    #[structopt(short = "-1", long, group = "server_or_client")]
    pub one_off: bool,
}

#[derive(Debug, StructOpt)]
pub struct ClientOpts {
    /// Run in client mode, connect to <host>
    #[structopt(short, long, group = "server_or_client")]
    pub client: Option<String>,
    /// Length of buffer to read or write, default is 2MiB (2097152)
    #[structopt(short, long)]
    pub length: Option<usize>,
    /// Time in seconds to transmit for
    #[structopt(short, long, default_value = "10")]
    pub time: u64,
    /// How many parallel streams used in testing
    #[structopt(short = "-P", long, default_value = "1")]
    pub parallel: u16,
}

#[derive(Debug, StructOpt)]
pub struct CommonOpts {
    /// Server port to listen on/connect to
    #[structopt(short, long, default_value = "7559")]
    pub port: u16,
    /// Seconds between periodic throughput reports
    #[structopt(short, long, default_value = "1")]
    pub interval: u16,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "netperf", 
    group = structopt::clap::ArgGroup::with_name("server_or_client").required(true),
    setting = structopt::clap::AppSettings::ColoredHelp)]
/// A network performance measurement tool
pub struct Opts {
    #[structopt(flatten)]
    pub server_opts: ServerOpts,
    #[structopt(flatten)]
    pub client_opts: ClientOpts,
    #[structopt(flatten)]
    pub common_opts: CommonOpts,
    #[structopt(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

// Exports
pub mod net_utils;
pub mod perf_test;
pub use perf_test::*;
pub mod consts;
pub mod control;
pub mod data;
pub mod stream_worker;
pub mod ui;
