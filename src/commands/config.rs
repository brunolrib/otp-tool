use anyhow::Result;
use clap::{command, Args, Subcommand};
use log::warn;
use crate::core::{config};

#[derive(Args, Debug)]
#[command(about = "Manage otp-tool configuration settings")]
pub struct Config {
    #[command(subcommand)]
    subcommand: ConfigSubcommands,
}

#[derive(Subcommand, Debug)]
enum ConfigSubcommands {
    List(ListConfig),
    Get(GetConfig),
    Set(SetConfig),
}

impl Config {
    pub fn run(&self) -> Result<()> {
        match &self.subcommand {
            ConfigSubcommands::List(list_config) => list_config.run(),
            ConfigSubcommands::Get(get_config) => get_config.run(),
            ConfigSubcommands::Set(set_config) => set_config.run(),
        }
    }
}

#[derive(Args, Debug)]
#[command(about = "Set a configuration value")]
struct SetConfig {
    #[arg(short, long, help = "Configuration key to set")]
    key: String,
    #[arg(short, long, help = "Value to set for the configuration key")]
    value: String,
}

impl SetConfig {
    fn run(&self) -> Result<()> {
        let mut cfg = config::read();
        match self.key.as_str() {
            "duration" => cfg.duration = self.value.parse::<u32>().unwrap(),
            "digits" => cfg.digits = self.value.parse::<u32>().unwrap(),
            "wait_for_next_time" => cfg.wait_for_next_time = self.value.parse::<u32>().unwrap(),
            _ => {
                println!("Unknown configuration key: {}", self.key);
                return Ok(());
            }
            
        }

        config::write(&cfg);
        Ok(())
    }
}

#[derive(Args, Debug)]
#[command(about = "Get a configuration value")]
struct GetConfig {
    #[arg(short, long, help = "Configuration key to get")]
    key: String,
}

impl GetConfig {
    fn run(&self) -> Result<()> {
        let cfg = config::read();
        let json = serde_json::to_string(&cfg).unwrap();
        let json: serde_json::Value = serde_json::from_str(&json).unwrap();

        if let Some(value) = json.get(&self.key) {
            println!("{}: {}", self.key, value);
        } else {
            warn!("Key not found: {}", self.key);
        }
        Ok(())
    }
}

#[derive(Args, Debug)]
#[command(about = "List all configuration values")]
struct ListConfig;

impl ListConfig {
    fn run(&self) -> Result<()> {
        let cfg = config::read();
        println!("Listing all configs:");
        
        let json = serde_json::to_string(&cfg).unwrap();
        let json: serde_json::Value = serde_json::from_str(&json).unwrap();
        for (key, value) in json.as_object().unwrap() {
            println!("{}: {}", key, value);
        }

        Ok(())
    }
}