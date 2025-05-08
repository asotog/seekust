use std::path::Path;
use seekust::scan_dir;
use clap::Parser;

/// This is a utility to search files and files content
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Pattern to search for
    keyword_pattern: String,

     /// Ignore list of patterns whether ignore folders or files
    #[arg(short, long, default_value_t = String::from(""))]
    ignore_pattern: String,
}

fn main() {
    let args = Args::parse();

    println!("Searching files containing: {}", args.keyword_pattern);
    
    if !scan_dir::find(Path::new("."), &args.keyword_pattern, &args.ignore_pattern) {
        println!("No matches found :(");
    }
}