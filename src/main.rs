use clap::{Parser, Subcommand};
use proxy_health::query_proxy_health;
use anyhow::{Result, Context};
use reqwest::Url;

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
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    match args.command {
        SubCommands::Health { name } => query_proxy_health(&name, &args.monitoring_api_key, &args.broker_url).await.context("Failed to query proxy health"),
    }
}
