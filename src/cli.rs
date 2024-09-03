pub mod commands;
pub mod models;

use clap::Parser as _;
use commands::{analyze, delete_channel, join_channel};
use grammers_client::Client;
use models::{Cli, Commands};

pub fn parse() -> Cli {
    Cli::parse()
}

pub async fn run(client: &Client, command: Commands) {
    match command {
        Commands::Analyze(config) => {
            analyze(config, client)
                .await
                .expect("Error while analyze chats");
        }
        Commands::Join(config) => {
            join_channel(config, client)
                .await
                .expect("Error while join channel/supergroup");
        }
        Commands::Delete(config) => {
            delete_channel(config, client)
                .await
                .expect("Error while join channel/supergroup");
        }
    };
}
