pub mod add;
pub mod config;
pub mod get;
pub mod list;
pub mod remove;
pub mod args;

use anyhow::Result;
use clap::{command, Parser, Subcommand};
use crate::commands::{
    add::AddEntry,
    remove::Remove,
    list::List,
    get::GetSecret,
    config::Config,
};

#[derive(Parser)]
#[command(name = "otp-tool")]
#[command(about = "Manage oathtool OTPs using encryption secrets", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add(AddEntry),
    Remove(Remove),
    List(List),
    Get(GetSecret),
    Config(Config),
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Add(cmd) => cmd.run(),
            Commands::Remove(cmd) => cmd.run(),
            Commands::List(cmd) => cmd.run(),
            Commands::Get(cmd) => cmd.run(),
            Commands::Config(cmd) => cmd.run(),
        }
    }
}