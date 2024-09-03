mod cli;
mod client;
mod configs;

use cli::{models::Cli, parse as cli_parse, run as cli_run};
use client::auth;
use configs::{read_raw_toml, Config};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

#[tokio::main]
async fn main() {
    let Cli {
        config_file_path,
        command,
    } = cli_parse();

    let raw = read_raw_toml(config_file_path).expect("Error while reading config file");
    let config = Config::parse_raw_toml(raw).expect("Error while parsing raw config");

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new(config.logging.directives))
        .init();

    info!("Init client");
    let client = auth::init(&config.client).await;

    info!("Authorize client");
    auth::authorize(&client, &config.client).await;

    info!("Client connected and authorized successfully");

    cli_run(&client, command).await;
}
