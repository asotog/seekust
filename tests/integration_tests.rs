use std::fs;
use tempfile::TempDir;

fn setup_test_files() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test file 1
    fs::write(
        temp_dir.path().join("test1.txt"),
        "Hello World\nThis is a test\nHello again",
    ).unwrap();
    
    // Create test file 2
    fs::write(
        temp_dir.path().join("test2.txt"),
        "Another test file\nWith Hello in it",
    ).unwrap();
    
    temp_dir
}

#[test]
fn test_find_single_match() {
    let temp_dir = setup_test_files();
    assert!(seekust::scan_dir::find(temp_dir.path(), "World", ""));
}

#[test]
fn test_find_multiple_matches() {
    let temp_dir = setup_test_files();
    assert!(seekust::scan_dir::find(temp_dir.path(), "Hello", ""));
}

#[test]
fn test_no_matches() {
    let temp_dir = setup_test_files();
    assert!(!seekust::scan_dir::find(temp_dir.path(), "NonexistentString", ""));
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    assert!(!seekust::scan_dir::find(temp_dir.path(), "test", ""));
}

#[test]
fn test_ignore_pattern() {
    let temp_dir = setup_test_files();
    assert!(seekust::scan_dir::find(temp_dir.path(), "World", ".git"));
}