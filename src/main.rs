//! This crate can be used to detect and report duplicate files in a file 
//! system.
//! 
//! The functions in [`duplicates`] can be used to get a list of lists of 
//! duplicate file paths. [`get_duplicates_hashed`] should generally be 
//! preferred, since it uses memory proportional to the size of the largest 
//! file, independent of how many files are being checked.
//! 
//! The function [`get_descendants`] is convenient for getting the list of 
//! all files which are descendants of a base path.
//! 
//! ## Basic usage:
//! ```no_run
//! // Assume the following directory structure, where the contents of 
//! // a.txt and e.txt are identical, and the contents of b.txt, c.txt, and
//! // d.txt are identical.
//! // 
//! // files
//! // ├── a.txt
//! // ├── b.txt
//! // └── more_files
//! //     ├── c.txt
//! //     ├── d.txt
//! //     └── even_more_files
//! //         ├── e.txt
//! //         └── f.txt
//! 
//! let base_path = Path::from("files");
//! let descendants = get_descendants(base_path);
//! let duplicates = get_duplicates_hashed(&descendants);
//! let expected = vec![
//!     vec!["files/a.txt", "files/more_files/even_more_files/e.txt"],
//!     vec!["files/b.txt", "files/more_files/c.txt", "files/more_files.d.txt"]
//! ];
//! assert!(duplicates == expected);
//! ```

use std::path::Path;

use clap::Parser;
use duplicates::{get_duplicates, get_duplicates_hashed};
use paths::get_descendants;

pub mod duplicates;
pub mod paths;

/// Command line arguments used when running this crate as a script.
#[derive(Parser)]
struct Cli {
    #[clap(default_value_t = String::from("."))]
    base_path: String,
}

/// Outputs all of the duplicate files from the descendants of a base_path.
///
/// See the (README.md) for usage details.
fn main() {
    let args = Cli::parse();
    let base_path = Path::new(&args.base_path);
    let descendants = get_descendants(base_path);
    for paths in get_duplicates_hashed(&descendants) {
        println!(
            "{}\n",
            paths
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}
