use std::fs;

use crate::error::MyError;
use crate::storage::DirectoryPath;

#[tauri::command]
pub fn read_directory(path: String) -> Result<Vec<DirectoryPath>, MyError> {
    let read_dir_result = fs::read_dir(path)?;

    let mut folder_structure: Vec<DirectoryPath> = Vec::new();

    for entry in read_dir_result {
        let entry = entry?;
        folder_structure.push(DirectoryPath::from(&entry));
    }

    Ok(folder_structure)
}

#[tauri::command]
pub fn open_file() -> Result<String, MyError> {
    std::fs::File::open("path/that/does/not/exist")?;
    // Return `null` on success
    Ok("Yash".to_string())
}
