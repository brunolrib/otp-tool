use std::process::Command;

pub fn generate(secret: &str, digits: u8, time: u8) -> String {
    let output = Command::new("oathtool")
        .arg(format!("--totp"))
        .arg(format!("--base32"))
        .arg(format!("-d {}", digits))
        .arg(format!("-s {}", time))
        .arg(secret)
        .output()
        .expect("Failed to execute oathtool");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_with_valid_secret() {
        // Test with a known Base32 secret
        let secret = "JBSWY3DPEHPK3PXP"; // "Hello World!" in Base32
        let digits = 6;
        let time = 30;
        
        let result = generate(secret, digits, time);
        
        // The result should be a 6-digit numeric string
        assert_eq!(result.len(), 6);
        assert!(result.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_with_different_digits() {
        let secret = "JBSWY3DPEHPK3PXP";
        let digits = 8;
        let time = 30;
        
        let result = generate(secret, digits, time);
        
        // The result should be an 8-digit numeric string
        assert_eq!(result.len(), 8);
        assert!(result.chars().all(|c| c.is_ascii_digit()));
    }

    #[test] 
    fn test_generate_consistent_output() {
        let secret = "JBSWY3DPEHPK3PXP";
        let digits = 6;
        let time = 30;
        
        // Generate multiple times in quick succession
        let result1 = generate(secret, digits, time);
        let result2 = generate(secret, digits, time);
        
        // Results should be the same since time window hasn't changed
        assert_eq!(result1, result2);
    }

    #[test]
    #[should_panic(expected = "Failed to execute oathtool")]
    fn test_generate_with_invalid_secret() {
        // This test assumes oathtool will fail with invalid input
        let secret = "INVALID_BASE32_!!!";
        let digits = 6;
        let time = 30;
        
        generate(secret, digits, time);
    }

    #[test]
    fn test_generate_parameter_formatting() {
        // We can't easily test the actual command execution, but we can test
        // that our function doesn't panic with various parameters
        let secret = "JBSWY3DPEHPK3PXP";
        
        // Test different digit counts
        for digits in [4, 6, 8] {
            let result = generate(secret, digits, 30);
            assert_eq!(result.len(), digits as usize);
        }
    }

    #[test]
    fn test_generate_with_longer_secret() {
        // Test with a longer Base32 secret
        let secret = "MFRGG43TKFRGC5DMMFRGK3TTMVTWK3TTNJQWC3LUHNVWS33OHMVQY2LUHNVWS33OOQQGQ43TMVTWK3DSMFRGK";
        let digits = 6;
        let time = 30;
        
        let result = generate(secret, digits, time);
        
        assert_eq!(result.len(), 6);
        assert!(result.chars().all(|c| c.is_ascii_digit()));
    }
}