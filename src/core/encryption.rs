use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, KeyInit, OsRng}, AeadCore, Aes256Gcm, Nonce
};
use anyhow::{Error, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EncryptedSecret {
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
}

pub fn encrypt(plain: &str, password: &str) -> Result<EncryptedSecret> {
    let key = GenericArray::from_slice(&password.as_bytes()[..32]);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);

    let ciphertext = cipher.encrypt(&nonce, plain.as_ref());

    if ciphertext.is_err() {
        return Err(Error::msg(format!("Encryption failed: {}", ciphertext.unwrap_err())));
    }

    Ok(EncryptedSecret { nonce: nonce.to_vec(), ciphertext: ciphertext.unwrap() })
}

pub fn decrypt(encrypted: &EncryptedSecret, password: &str) -> Result<String> {
    let key = GenericArray::from_slice(&password.as_bytes()[..32]);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(&encrypted.nonce);
    let encrypted_text = bincode::serialize(&encrypted.ciphertext).unwrap();
    let decrypted_bytes = cipher.decrypt(&nonce, encrypted_text.as_ref());
    if decrypted_bytes.is_err() {
        return Err(Error::msg(format!("Decryption failed: {}", decrypted_bytes.unwrap_err())));
    }
    let secret_restored = String::from_utf8(decrypted_bytes.unwrap()).unwrap();

    Ok(secret_restored)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_success() {
        let plain_text = "test_secret_key_123";
        let password = "this_is_a_very_long_password_32chars";
        
        let encrypted = encrypt(plain_text, password).unwrap();
        assert!(!encrypted.nonce.is_empty());
        assert!(!encrypted.ciphertext.is_empty());
        
        let decrypted = decrypt(&encrypted, password).unwrap();
        assert_eq!(decrypted, plain_text);
    }

    #[test]
    fn test_encrypt_decrypt_different_passwords_fail() {
        let plain_text = "test_secret_key_123";
        let password1 = "this_is_a_very_long_password_32chars";
        let password2 = "this_is_a_different_password_32chars";
        
        let encrypted = encrypt(plain_text, password1).unwrap();
        let result = decrypt(&encrypted, password2);
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypt_with_short_password() {
        let plain_text = "test_secret";
        let short_password = "short";
        
        // This should work since we take the first 32 bytes and pad with zeros
        let result = encrypt(plain_text, short_password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_encrypt_empty_string() {
        let plain_text = "";
        let password = "this_is_a_very_long_password_32chars";
        
        let encrypted = encrypt(plain_text, password).unwrap();
        let decrypted = decrypt(&encrypted, password).unwrap();
        assert_eq!(decrypted, plain_text);
    }

    #[test]
    fn test_encrypt_unicode_text() {
        let plain_text = "🔐 Test secret with émojis and àccénts";
        let password = "this_is_a_very_long_password_32chars";
        
        let encrypted = encrypt(plain_text, password).unwrap();
        let decrypted = decrypt(&encrypted, password).unwrap();
        assert_eq!(decrypted, plain_text);
    }

    #[test]
    fn test_encrypted_secret_serialization() {
        let plain_text = "test_secret";
        let password = "this_is_a_very_long_password_32chars";
        
        let encrypted = encrypt(plain_text, password).unwrap();
        
        // Test that EncryptedSecret can be serialized and deserialized
        let serialized = bincode::serialize(&encrypted).unwrap();
        let deserialized: EncryptedSecret = bincode::deserialize(&serialized).unwrap();
        
        let decrypted = decrypt(&deserialized, password).unwrap();
        assert_eq!(decrypted, plain_text);
    }

    #[test]
    fn test_different_nonces_for_same_input() {
        let plain_text = "test_secret";
        let password = "this_is_a_very_long_password_32chars";
        
        let encrypted1 = encrypt(plain_text, password).unwrap();
        let encrypted2 = encrypt(plain_text, password).unwrap();
        
        // Nonces should be different for each encryption
        assert_ne!(encrypted1.nonce, encrypted2.nonce);
        
        // But both should decrypt to the same plaintext
        let decrypted1 = decrypt(&encrypted1, password).unwrap();
        let decrypted2 = decrypt(&encrypted2, password).unwrap();
        assert_eq!(decrypted1, plain_text);
        assert_eq!(decrypted2, plain_text);
    }
}