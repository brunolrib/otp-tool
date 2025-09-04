use anyhow::Result;
use clap::{command, Args};

use crate::core::file_system;

#[derive(Args, Debug)]
#[command(about = "List all OTP secrets")]
pub struct List {}

impl List {
    pub fn run(&self) -> Result<()> {
        let app_dir = file_system::get_app_data_dir().unwrap();
        let secret_path = format!("{}/secrets/", app_dir);

        let files_names = file_system::list_files_in_dir(&secret_path)?;
        if files_names.is_empty() {
            println!("No OTP secrets found.");
            return Ok(());
        }
        
        for file_name in files_names {
            println!("Found OTP secret: {}", file_name);
        }
        Ok(())
    }
}