// Copyright (C) 2022 The Elabs Project Authors.
// This file is part of the Elabs library.
//
// The Elabs library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// The Elabs library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with The Elabs library.
// If not, see <https://www.gnu.org/licenses/>.

use std::{env, path::PathBuf};

/// Get absolute path from a string.
/// # Arguments
/// * `path` - The path to get absolute path from.
/// # Returns
/// * The absolute path.
pub fn get_absolute_path(path: &str) -> String {
    if path.is_empty() {
        return "".to_string();
    }

    let mut path = path.to_string();

    if !path.starts_with("/") {
        path = env::current_dir()
            .unwrap()
            .join(path)
            .to_str()
            .unwrap()
            .to_string();
    }

    if path.ends_with("/.") {
        path.pop();
    }

    if path.ends_with("/") {
        path.pop();
    }

    path
}

/// get last folder name from a path.
/// # Arguments
/// * `path` - The path to get last folder name from.
/// # Returns
/// * The last folder name.
pub fn get_folder_name(path: &str) -> String {
    let pb = PathBuf::from(get_absolute_path(path));
    pb.file_name().unwrap().to_str().unwrap().to_string()
}

/// check if directory exists.
/// # Arguments
/// * `path` - The path to check.
/// # Returns
/// * `true` if the directory exists.
/// * `false` if the directory does not exist.
pub fn directory_exists(path: &str) -> bool {
    let pb = PathBuf::from(get_absolute_path(path));
    pb.is_dir()
}

/// check if file exists.
/// # Arguments
/// * `path` - The path to check.
/// # Returns
/// * `true` if the file exists.
/// * `false` if the file does not exist.
pub fn file_exists(path: &str) -> bool {
    let pb = PathBuf::from(get_absolute_path(path));
    pb.is_file()
}

/// Create a directory.
/// # Arguments
/// * `path` - The path to create.
/// # Returns
/// * `Ok(())` if the directory is created.
/// * `Err(String)` if the directory cannot be created.
pub fn create_directory(path: &str) -> Result<(), String> {
    let pb = PathBuf::from(get_absolute_path(path));
    match std::fs::create_dir_all(&pb) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}

/// Create a file.
/// # Arguments
/// * `path` - The path to create.
/// * `content` - The content of the file.
/// # Returns
/// * `Ok(())` if the file is created.
/// * `Err(String)` if the file cannot be created.
pub fn write_file(path: &str, content: &str) -> Result<(), String> {
    let pb = PathBuf::from(get_absolute_path(path));
    match std::fs::write(&pb, content) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}

/// Read a directory.
/// It will get all files and folders from the directory.
/// # Arguments
/// * `path` - The path to read.
/// # Returns
/// * `Vec<String>` - The files and folders.
pub fn read_directory(path: &str) -> Vec<String> {
    let pb = PathBuf::from(get_absolute_path(path));
    let mut files = Vec::new();

    if pb.is_dir() {
        for entry in std::fs::read_dir(&pb).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let path = path.to_str().unwrap();
            files.push(path.to_string());
        }
    }

    files
}

/// Read a file.
/// # Arguments
/// * `path` - The path to read.
/// # Returns
/// * `String` - The content of the file.
/// * `Err(String)` if the file cannot be read.
pub fn read_file(path: &str) -> Result<String, String> {
    let pb = PathBuf::from(get_absolute_path(path));
    match std::fs::read_to_string(&pb) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("{}", e)),
    }
}

/// Delete a file.
/// # Arguments
/// * `path` - The path to delete.
/// # Returns
/// * `true` if the file was deleted.
/// * `false` if the file was not deleted.
pub fn delete_file(path: &str) -> bool {
    let pb = PathBuf::from(get_absolute_path(path));
    std::fs::remove_file(&pb).is_ok()
}

/// Delete a directory.
/// # Arguments
/// * `path` - The path to delete.
/// # Returns
/// * `true` if the directory was deleted.
/// * `false` if the directory was not deleted.
pub fn delete_directory(path: &str) -> bool {
    let pb = PathBuf::from(get_absolute_path(path));
    std::fs::remove_dir_all(&pb).is_ok()
}
