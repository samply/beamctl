use std::process::ExitCode;

use clap::{Parser, Subcommand};
use icinga::IcingaCode;
use proxy_health::query_proxy_health;
use bridgehead_health::check_bridgehead;
use anyhow::Context;
use reqwest::Url;

mod icinga;
mod proxy_health;
mod bridgehead_health;

#[derive(Debug, Parser)]
#[clap(
    name("🌈 Samply.beamctl"),
    version,
    arg_required_else_help(true),
)]
struct CliArgs {
    
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Health {
        #[arg(long, env, value_parser)]
        monitoring_api_key: String,
        
        #[arg(long, env, value_parser)]
        broker_url: Url,
        name: String,
    },
    Bridgehead(bridgehead_health::BridgeheadCheck)
    
}


#[tokio::main]
async fn main() -> ExitCode {
    let args = CliArgs::parse();

    let result = match args.command {
        SubCommands::Health { name, monitoring_api_key, broker_url } => query_proxy_health(&name, &monitoring_api_key, &broker_url).await.context("Failed to query proxy health"),
        SubCommands::Bridgehead(bridgehead_check) => check_bridgehead(bridgehead_check).await,
    };
    match result {
        Err(e) => {
            println!("{e}");
            IcingaCode::Unknown
        },
        Ok(code) => code,
    }.into()
}
