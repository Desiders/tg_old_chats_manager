use clap::{builder::Styles, command, Args, Parser, Subcommand};
use clap_cargo::style;
use std::path::PathBuf;

pub const CLAP_STYLING: Styles = Styles::styled()
    .header(style::HEADER)
    .usage(style::USAGE)
    .literal(style::LITERAL)
    .placeholder(style::PLACEHOLDER)
    .error(style::ERROR)
    .valid(style::VALID)
    .invalid(style::INVALID);

#[derive(Parser)]
#[command(styles = CLAP_STYLING)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(
        short = 'c',
        long = "config-path",
        default_value = "configs/config.toml"
    )]
    pub config_file_path: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze chats
    Analyze(Analyze),
    /// Join chat
    Join(Join),
    /// Delete chat
    Delete(Delete),
}

#[derive(Debug, Args)]
pub struct Analyze {
    /// Analyze joined chats
    #[arg(short = 'j', long = "joined", default_value = "false")]
    pub joined: bool,
    /// Analyze left chats
    #[arg(short = 'l', long = "left", default_value = "false")]
    pub left: bool,
}

#[derive(Debug, Args)]
pub struct Join {
    /// Channel/supergroup ID to join
    #[arg(short = 'i', long = "id")]
    pub id: i64,
    /// Access hash of the channel/supergroup. It's required for most cases.
    #[arg(short = 'a', long = "access-hash")]
    pub access_hash: Option<i64>,
}

#[derive(Debug, Args)]
pub struct Delete {
    /// Channel/supergroup ID to delete
    #[arg(short = 'i', long = "id")]
    pub id: i64,
    /// Access hash of the channel/supergroup. It's required for most cases.
    #[arg(short = 'a', long = "access-hash")]
    pub access_hash: Option<i64>,
}
