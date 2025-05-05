use std::fs;
use std::path::Path;
use crate::file_utils;

/// Recursively scans a directory for files containing a specific string
/// 
/// # Arguments
/// * `dir` - The directory path to start scanning from
/// * `search_str` - The string to search for in files
/// 
/// # Returns
/// * `bool` - Indicates if matches were found
pub fn find(dir: &Path, search_str: &str) -> bool {
    let mut found_matches = false;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(matches) = file_utils::search_in_file(&entry.path(), search_str) {
                            found_matches = true;
                            println!("File: {}", entry.path().display());
                            for m in matches {
                                println!("  Line {}, Col {}", m.line, m.column);
                            }
                        }
                    } else if file_type.is_dir() {
                        if find(&entry.path(), search_str) {
                            found_matches = true;
                        }
                    }
                }
            }
        }
    }

    found_matches
}
