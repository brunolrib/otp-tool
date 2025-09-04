use anyhow::Result;
use clap::{command, Args};

use crate::{commands::args::PassArgs, core::{encryption, file_system, keyring, oathtool}};

#[derive(Args, Debug)]
#[command(about = "Get the OTP for a specific entry")]
pub struct GetSecret {
    #[command(flatten)]
    pub args: PassArgs,
}

impl GetSecret {
    pub fn run(&self) -> Result<()> {
        let password = keyring::handle_password(&self.args)?;
        let app_dir = file_system::get_app_data_dir()?;
        let secret_path = app_dir + format!("{}.bin", self.args.service_name).as_str();

        let secret_bin = file_system::read_bin(&secret_path)?;
        let encrypted = bincode::deserialize::<encryption::EncryptedSecret>(&secret_bin)?;
        let secret = encryption::decrypt(&encrypted, &password).unwrap();

        let otp = oathtool::generate(&secret, 6, 30);
        arboard::Clipboard::new()?.set_text(otp.clone())?;

        println!("OTP for {}: {}", self.args.service_name, otp);
        println!("OTP copied to clipboard.");
        Ok(())
    }
}