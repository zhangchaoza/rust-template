use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use once_cell::sync::Lazy;
use std::{
    net::{SocketAddr, SocketAddrV4},
    path::PathBuf,
    str::FromStr,
};

mod subcommand1;

/// global cli object
static CLI: Lazy<Cli> = Lazy::new(Cli::parse);

pub(crate) fn run<F>(on_parsed: F) -> Result<()>
where
    F: FnOnce(&Cli),
{
    let cli = &CLI;
    on_parsed(cli);
    match &cli.command {
        Some(command) => match command {
            Commands::SubCommand1 { .. } => subcommand1::run(),
        },
        _ => unreachable!(),
    }
}

#[derive(Parser, Debug, PartialEq)]
#[command(author, about)]
#[command(version = crate::versions::VERSION.as_str())]
#[command(long_about = None)]
pub(crate) struct Cli {
    /// log verbose
    #[arg(short, default_value_t=2, action = clap::ArgAction::Count ,value_parser = clap::value_parser!(u8).range(0..5))]
    pub verbose: u8,

    /// Log file name.
    #[arg(short, long)]
    pub log: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// Slave mode
    SubCommand1 {
        /// i32 arg
        #[arg(short, long, value_name = "connection type")]
        name: i32,

        /// optional arg
        #[arg(long, value_name = "points")]
        path: Option<PathBuf>,

        /// default value arg
        #[arg(short, long, value_name = "local")]
        #[arg(default_value_t = SocketAddr::V4(SocketAddrV4::from_str("0.0.0.0:80").unwrap()))]
        ip: SocketAddr,

        /// Enum arg
        #[arg(short, long, value_name = "connection type")]
        connection: SimpleType,

        /// Parse arg.
        #[arg(short, long,value_parser=string_to_i32)]
        script: Option<i32>,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum SimpleType {
    A,
    B,
}

fn string_to_i32(s: &str) -> Result<i32, String> {
    i32::from_str(s).map_err(|a| a.to_string())
}
