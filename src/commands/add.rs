use anyhow::Result;
use clap::{command, Args};
use log::info;

use crate::{commands::args::PassArgs, core::{encryption, file_system, keyring}};

#[derive(Args, Debug)]
#[command(about = "Add a new OTP entry for <name> encrypting <secret>")]
pub struct AddEntry {
    #[command(flatten)]
    pub args: PassArgs,

    #[arg(short, long, help = "Secret for the OTP entry")]
    secret: String,
}

impl AddEntry {
    pub fn run(&self) -> Result<()> {
        let password = keyring::handle_password(&self.args)?;

        let app_dir = file_system::get_app_data_dir().unwrap();
        let secret_path = app_dir + format!("secrets/{}.bin", self.args.service_name).as_str();
        let encrypted = encryption::encrypt(&self.secret, &password);
        let encrypted_bin = bincode::serialize(&encrypted.unwrap()).unwrap();

        file_system::write_bin(&secret_path, &encrypted_bin)?;
        info!("Added entry: {}", self.args.service_name);
        Ok(())
    }
}