use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use duplicates::get_duplicates_hashed;

pub mod duplicates;

fn get_descendants(base_path: &Path) -> Vec<PathBuf> {
    if base_path.is_dir() {
        match fs::read_dir(base_path) {
            Ok(dir_iter) => {
                let mut result = Vec::new();
                for dir_entry in dir_iter {
                    if let Ok(dir_entry) = dir_entry {
                        result.extend(get_descendants(&dir_entry.path()));
                    }
                }
                result
            }
            _ => Vec::new(),
        }
    } else {
        vec![PathBuf::from(base_path)]
    }
}

#[derive(Parser)]
struct Cli {
    #[clap(default_value_t = String::from("."))]
    base_path: String,
}

fn main() {
    let args = Cli::parse();
    let base_path = Path::new(&args.base_path);
    let descendants = get_descendants(base_path);
    for paths in get_duplicates_hashed(&descendants) {
        println!(
            "{}",
            paths
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}
