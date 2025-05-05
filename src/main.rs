use std::env;
use std::path::Path;
use seekust::scan_dir;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Searching files containing: {}", args[1]);
    
    if !scan_dir::find(Path::new("."), &args[1]) {
        println!("No matches found :(");
    }
}