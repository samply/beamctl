use std::process::ExitCode;

use clap::{Parser, Subcommand};
use icinga::IcingaCode;
use proxy_health::query_proxy_health;
use anyhow::Context;
use reqwest::Url;

mod icinga;
mod proxy_health;

#[derive(Debug, Parser)]
#[clap(
    name("🌈 Samply.beamctl"),
    version,
    arg_required_else_help(true),
)]
struct CliArgs {
    #[clap(long, env, value_parser)]
    monitoring_api_key: String,

    #[clap(long, env, value_parser)]
    broker_url: Url,

    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Health {
        name: String,
    }
}


#[tokio::main]
async fn main() -> ExitCode {
    let args = CliArgs::parse();

    let result = match args.command {
        SubCommands::Health { name } => query_proxy_health(&name, &args.monitoring_api_key, &args.broker_url).await.context("Failed to query proxy health"),
    };
    let exit_code = if let Err(e) = result {
        eprint!("{e}");
        IcingaCode::Unknown
    } else {
        IcingaCode::Ok
    };
    exit_code.into()
}
