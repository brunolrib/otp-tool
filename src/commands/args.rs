use clap::{arg, Args};

#[derive(Debug, Args)]
pub struct PassArgs {
    #[arg(name = "name", short = 'n', long, required = true, help = "Name of the service")]
    pub service_name: String,

    #[arg(short, long, global = true, help = "Password for managing secrets encryption")]
    pub password: Option<String>,

    #[arg(long, global = true, default_value = "false", help = "Use stored password from keyring")]
    pub use_stored: Option<bool>,
}

impl Default for PassArgs {
    fn default() -> Self {
        PassArgs {
            service_name: String::new(),
            password: None,
            use_stored: Some(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_args_default() {
        let args = PassArgs::default();
        
        assert_eq!(args.service_name, String::new());
        assert_eq!(args.password, None);
        assert_eq!(args.use_stored, Some(false));
    }

    #[test]
    fn test_pass_args_creation() {
        let args = PassArgs {
            service_name: "gmail".to_string(),
            password: Some("secret123".to_string()),
            use_stored: Some(true),
        };
        
        assert_eq!(args.service_name, "gmail");
        assert_eq!(args.password, Some("secret123".to_string()));
        assert_eq!(args.use_stored, Some(true));
    }

    #[test]
    fn test_pass_args_no_password() {
        let args = PassArgs {
            service_name: "github".to_string(),
            password: None,
            use_stored: Some(false),
        };
        
        assert_eq!(args.service_name, "github");
        assert_eq!(args.password, None);
        assert_eq!(args.use_stored, Some(false));
    }

    #[test]
    fn test_pass_args_use_stored_none() {
        let args = PassArgs {
            service_name: "twitter".to_string(),
            password: Some("pass456".to_string()),
            use_stored: None,
        };
        
        assert_eq!(args.service_name, "twitter");
        assert_eq!(args.password, Some("pass456".to_string()));
        assert_eq!(args.use_stored, None);
    }

    #[test]
    fn test_pass_args_debug_format() {
        let args = PassArgs {
            service_name: "test".to_string(),
            password: Some("secret".to_string()),
            use_stored: Some(true),
        };
        
        let debug_str = format!("{:?}", args);
        assert!(debug_str.contains("service_name"));
        assert!(debug_str.contains("password"));
        assert!(debug_str.contains("use_stored"));
    }
}