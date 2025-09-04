
use clap::Parser;
use anyhow::{Result};

mod core;
mod commands;


fn main() -> Result<()> {
    env_logger::init();
    let cmd = commands::Cli::parse();

    // Run git-cliff
    let mut exit_code = 0;
    if let Err(e) = cmd.run() {
        log::error!("{}", e);
        exit_code = 1;
    }

    std::process::exit(exit_code);
}
