use std::fs;
use std::path::Path;

pub struct Match {
    pub line: usize,
    pub column: usize,
    pub line_content: String,
}

/// Searches for occurrences of a string in a file and returns their locations
///
/// # Arguments
/// * `file_path` - The path to the file to search in
/// * `search_str` - The string to search for
///
/// # Returns
/// * `Some(Vec<Match>)` - A vector of matches if any are found
/// * `None` - If the file cannot be read or no matches are found
pub fn search_in_file(file_path: &Path, search_str: &str) -> Option<Vec<Match>> {
    fs::read_to_string(file_path).ok().and_then(|contents| {
        if !contents.contains(search_str) {
            return None;
        }
        
        let matches: Vec<Match> = contents
            .lines()
            .enumerate()
            .filter_map(|(line_num, line)| {
                line.find(search_str).map(|col| Match {
                    line: line_num + 1,
                    column: col + 1,
                    line_content: line.to_string(),
                })
            })
            .collect();
        
        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    })
}
