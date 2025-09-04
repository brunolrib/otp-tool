use anyhow::{Context, Result};
use std::{env, fs::File, io::{Read, Write}};
use crate::core::config::{APP_NAME};

pub fn read_bin(file_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).with_context(
        || format!("Failed to read from file: {}", file_path)
    )?;
    Ok(contents)
}

pub fn write_bin(file_path: &str, contents: &Vec<u8>) -> Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(contents).with_context(
        || format!("Failed to write to file: {}", file_path)
    )?;
    Ok(())
}

pub fn delete_file(file_path: &str) -> Result<()> {
    std::fs::remove_file(file_path).with_context(
        || format!("Failed to delete file: {}", file_path)
    )?;
    Ok(())
}

pub fn exists(file_path: &str) -> Result<bool> {
    Ok(std::path::Path::new(file_path).exists())
}

pub fn list_files_in_dir(dir_path: &str) -> Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            files.push(entry.file_name().into_string().unwrap());
        }
    }
    Ok(files)
}

pub fn get_app_data_dir() -> Result<String> {
    let home_dir = env::home_dir().with_context(
        || "Failed to get home directory"
    )?;

    let app_data_dir = home_dir.join(format!(".{}", APP_NAME));
    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir)?;
    }
    Ok(app_data_dir.to_str().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::path::PathBuf;

    fn create_test_file(dir: &TempDir, name: &str, content: &[u8]) -> PathBuf {
        let file_path = dir.path().join(name);
        std::fs::write(&file_path, content).unwrap();
        file_path
    }

    #[test]
    fn test_read_write_bin() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.bin");
        let file_path_str = file_path.to_str().unwrap();
        
        let test_data = vec![1, 2, 3, 4, 5, 255, 0, 127];
        
        // Write binary data
        write_bin(file_path_str, &test_data).unwrap();
        
        // Read binary data
        let read_data = read_bin(file_path_str).unwrap();
        
        assert_eq!(test_data, read_data);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_bin("/nonexistent/file.bin");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_nonexistent_directory() {
        let result = write_bin("/nonexistent/directory/file.bin", &vec![1, 2, 3]);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = create_test_file(&temp_dir, "test_delete.txt", b"test content");
        let file_path_str = file_path.to_str().unwrap();
        
        // Verify file exists
        assert!(file_path.exists());
        
        // Delete file
        delete_file(file_path_str).unwrap();
        
        // Verify file no longer exists
        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_nonexistent_file() {
        let result = delete_file("/nonexistent/file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_exists() {
        let temp_dir = TempDir::new().unwrap();
        let existing_file = create_test_file(&temp_dir, "existing.txt", b"content");
        let nonexistent_file = temp_dir.path().join("nonexistent.txt");
        
        assert!(exists(existing_file.to_str().unwrap()).unwrap());
        assert!(!exists(nonexistent_file.to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_list_files_in_dir() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create some test files
        create_test_file(&temp_dir, "file1.txt", b"content1");
        create_test_file(&temp_dir, "file2.bin", b"content2");
        create_test_file(&temp_dir, "file3.dat", b"content3");
        
        // Create a subdirectory (should be ignored)
        let sub_dir = temp_dir.path().join("subdir");
        std::fs::create_dir(&sub_dir).unwrap();
        
        let files = list_files_in_dir(temp_dir.path().to_str().unwrap()).unwrap();
        
        assert_eq!(files.len(), 3);
        assert!(files.contains(&"file1.txt".to_string()));
        assert!(files.contains(&"file2.bin".to_string()));
        assert!(files.contains(&"file3.dat".to_string()));
    }

    #[test]
    fn test_list_files_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let files = list_files_in_dir(temp_dir.path().to_str().unwrap()).unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_list_files_nonexistent_dir() {
        let result = list_files_in_dir("/nonexistent/directory");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_app_data_dir() {
        let app_dir = get_app_data_dir().unwrap();
        
        // Should contain the app name
        assert!(app_dir.contains(APP_NAME));
        
        // The directory should exist after calling the function
        assert!(std::path::Path::new(&app_dir).exists());
    }

    #[test]
    fn test_write_read_empty_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("empty.bin");
        let file_path_str = file_path.to_str().unwrap();
        
        let empty_data = vec![];
        
        write_bin(file_path_str, &empty_data).unwrap();
        let read_data = read_bin(file_path_str).unwrap();
        
        assert_eq!(empty_data, read_data);
    }
}
