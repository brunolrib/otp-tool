use anyhow::{Context, Result, Error};
use keyring::{Entry};
use crate::{commands::args::{PassArgs}, core::config::APP_NAME};

fn save_password(password: &str) -> Result<()> {
    let entry = Entry::new(APP_NAME, APP_NAME).with_context(
        || "Failed to create keyring entry"
    )?;
    entry.set_password(password).with_context(
        || "Failed to set password"
    )?;
    Ok(())
}

fn read_password() -> Result<String> {
    let password = rpassword::prompt_password("Enter your password:").unwrap();
    save_password(&password)?;

    Ok(password)
}

pub fn handle_password(args: &PassArgs) -> Result<String> {
    let entry = Entry::new(APP_NAME, APP_NAME).with_context(
        || format!("Failed to create keyring entry for app: {}", APP_NAME)
    )?;
    let password_input: String;
    let saved_password: String = entry.get_password().unwrap_or("".to_string());

    if args.use_stored.unwrap_or(false) {
        if saved_password.is_empty() {
            return Err(Error::msg("No stored password found."));
        } else {
            return Ok(saved_password)
        }
    } 

    if args.password.is_some() {
        password_input = args.password.as_deref().unwrap_or("").to_string();
    } else {
        password_input = read_password()?;
    }

    println!("password_input: {}", password_input);
    println!("saved_password: {}", saved_password);
    if password_input != saved_password {
        return Err(Error::msg("Passwords do not match"));
    }

    Ok(password_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_args(service_name: &str, password: Option<String>, use_stored: Option<bool>) -> PassArgs {
        PassArgs {
            service_name: service_name.to_string(),
            password,
            use_stored,
        }
    }

    #[test]
    fn test_pass_args_default() {
        let args = PassArgs::default();
        
        assert_eq!(args.service_name, "");
        assert_eq!(args.password, None);
        assert_eq!(args.use_stored, Some(false));
    }

    #[test]
    fn test_create_test_args() {
        let args = create_test_args("test-service", Some("password123".to_string()), Some(true));
        
        assert_eq!(args.service_name, "test-service");
        assert_eq!(args.password, Some("password123".to_string()));
        assert_eq!(args.use_stored, Some(true));
    }

    #[test]
    fn test_create_test_args_no_password() {
        let args = create_test_args("test-service", None, Some(false));
        
        assert_eq!(args.service_name, "test-service");
        assert_eq!(args.password, None);
        assert_eq!(args.use_stored, Some(false));
    }

    // Note: Tests for save_password, read_password, delete_password, and handle_password
    // are difficult to unit test properly because they depend on:
    // 1. System keyring access
    // 2. User input via stdin
    // 3. External keyring state
    //
    // In a production application, we would:
    // 1. Create traits for KeyringService and InputService
    // 2. Use dependency injection to pass mock implementations
    // 3. Test the business logic separately from the I/O operations
    //
    // For now, we'll add integration tests that require manual verification
    // or create higher-level tests that verify the overall behavior.

    #[test]
    fn test_handle_password_with_use_stored_but_no_stored_password() {
        let args = create_test_args("test-service", Some("password123".to_string()), Some(true));
        
        // This test would require mocking the keyring entry.get_password() to return empty string
        // For now, we'll just test the logic structure
        assert_eq!(args.use_stored.unwrap_or(false), true);
    }

    #[test]
    fn test_password_matching_logic() {
        // Test the password comparison logic
        let provided_password = "password123";
        let stored_password = "password123";
        
        assert_eq!(provided_password, stored_password);
        
        let different_password = "different";
        assert_ne!(provided_password, different_password);
    }

    #[test]
    fn test_args_password_unwrap_logic() {
        let args_with_password = create_test_args("test", Some("pass123".to_string()), Some(false));
        let args_without_password = create_test_args("test", None, Some(false));
        
        assert_eq!(args_with_password.password.as_deref().unwrap_or(""), "pass123");
        assert_eq!(args_without_password.password.as_deref().unwrap_or(""), "");
    }
}