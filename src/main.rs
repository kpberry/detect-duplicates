use std::path::Path;

use clap::Parser;
use duplicates::{paths::get_descendants, duplicates::get_duplicates_hashed};

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
