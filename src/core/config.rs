use std::env;
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

pub const APP_NAME: &str = "otp-tool";
const DEFAULT_TIME: u32 = 30;
const DEFAULT_DIGITS: u32 = 6;
const REMAINING_WAIT_FOR_NEXT: u32 = 5;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub duration: u32,
    pub digits: u32,
    pub wait_for_next_time: u32,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self { Self { 
        duration: DEFAULT_TIME,
        digits: DEFAULT_DIGITS,
        wait_for_next_time: REMAINING_WAIT_FOR_NEXT
    } }
}

pub fn read() -> AppConfig {
    let x = confy::load_path(get_config_path().unwrap()).unwrap();
    Ok(x)
        .unwrap_or_else(|_| AppConfig::default())
}

pub fn write(config: &AppConfig) -> () {
    confy::store_path(get_config_path().unwrap(), config).unwrap();
}

fn get_config_path() -> Result<String> {
    let mut path = env::home_dir().unwrap();
    path.push(format!(".{APP_NAME}/{APP_NAME}.yaml"));
    match path.to_str() {
        Some(s) => Ok(s.to_string()),
        None => Err(anyhow::anyhow!("Failed to convert path to string")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        
        assert_eq!(config.duration, DEFAULT_TIME);
        assert_eq!(config.digits, DEFAULT_DIGITS);
        assert_eq!(config.wait_for_next_time, REMAINING_WAIT_FOR_NEXT);
    }

    #[test]
    fn test_app_config_serialization() {
        let config = AppConfig {
            duration: 60,
            digits: 8,
            wait_for_next_time: 10,
        };
        
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(config.duration, deserialized.duration);
        assert_eq!(config.digits, deserialized.digits);
        assert_eq!(config.wait_for_next_time, deserialized.wait_for_next_time);
    }

    #[test]
    fn test_constants() {
        assert_eq!(APP_NAME, "otp-tool");
        assert_eq!(DEFAULT_TIME, 30);
        assert_eq!(DEFAULT_DIGITS, 6);
        assert_eq!(REMAINING_WAIT_FOR_NEXT, 5);
    }

    #[test]
    fn test_read_config_when_none_exists() {
        // This test reads the actual config, so we need to be careful
        // In a real scenario, we'd want to mock the config path
        let config = read();
        
        // Should have default values if no config exists
        assert!(config.duration > 0);
        assert!(config.digits > 0);
        assert!(config.wait_for_next_time > 0);
    }

    #[test]
    fn test_get_config_path() {
        let path = get_config_path().unwrap();
        
        assert!(path.contains(APP_NAME));
        assert!(path.ends_with(".yaml"));
    }

    #[test]
    fn test_app_config_custom_values() {
        let config = AppConfig {
            duration: 45,
            digits: 8,
            wait_for_next_time: 7,
        };
        
        assert_eq!(config.duration, 45);
        assert_eq!(config.digits, 8);
        assert_eq!(config.wait_for_next_time, 7);
    }
}
