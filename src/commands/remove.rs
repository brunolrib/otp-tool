use anyhow::Result;
use clap::{command, Args};
use crate::{commands::args::PassArgs, core::file_system, core::keyring};

#[derive(Args, Debug)]
#[command(about = "Remove an existing OTP entry")]
pub struct Remove {
    #[command(flatten)]
    pub args: PassArgs,
}

impl Remove {
    pub fn run(&self) -> Result<()> {
        if !file_system::exists(&self.args.service_name)? {
            return Err(anyhow::anyhow!("Service not found: {}", self.args.service_name));
        }

        keyring::delete_password(&self.args)?;

        let app_dir = file_system::get_app_data_dir().unwrap();
        let secret_path = format!("{}/secrets/", app_dir);
        
        let file_path = format!("{}{}", secret_path, self.args.service_name);
        file_system::delete_file(&file_path).unwrap();

        Ok(())
    }
}