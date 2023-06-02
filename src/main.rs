use clap::{Parser, Subcommand};
use proxy_health::query_proxy_health;

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

    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Proxy {
        name: String,
    }
}


#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    match args.command {
        SubCommands::Proxy { name } => query_proxy_health(&name, &args.monitoring_api_key, None).await,
    }
}
