use std::path::{Path, PathBuf, MAIN_SEPARATOR_STR};

use clap::Parser;
use colored::{Color, Colorize};
use duplicates::{copies::get_copies_hashed, paths::{get_common_prefix, get_descendants}};

/// Command line arguments used when running this crate as a script.
#[derive(Parser)]
struct Cli {
    #[clap()]
    paths: Vec<String>,
    #[clap(long, default_value_t = 2)]
    min_count: usize,
    #[clap(long)]
    max_count: Option<usize>,
    #[clap(short, long, default_value_t = String::from("\n"))]
    separator: String,
    #[clap(short, long, default_value_t = String::from("\n"))]
    group_separator: String,
    #[clap(short, long)]
    display_count: bool,
    #[clap(short, long)]
    max_depth: Option<usize>,
    #[clap(short, long)]
    no_color_suffixes: bool
}

/// Outputs all of the duplicate files from the descendants of a base_path.
///
/// See the (README.md) for usage details.
fn main() {
    let args = Cli::parse();
    let base_paths = args.paths.iter().map(|s| Path::new(s));
    let descendants: Vec<PathBuf> = base_paths
        .map(|path| get_descendants(path, args.max_depth))
        .flatten()
        .collect();
    let (copies, errors) = get_copies_hashed(&descendants);
    for paths in copies {
        let count = paths.len();
        if count < args.min_count || args.max_count.is_some_and(|max_count| count > max_count) {
            continue;
        }

        let path_strings: Vec<String> = if args.no_color_suffixes {
            paths.iter().map(|path| path.display().to_string()).collect()
        } else {
            let common_prefix = get_common_prefix(&paths);
            let common_prefix_string = common_prefix.display().to_string().color(Color::Cyan);
            paths.iter().map(|path| {
                let stripped = path.strip_prefix(&common_prefix);
                if let Ok(stripped) = stripped {
                    format!("{}{}{}", common_prefix_string, MAIN_SEPARATOR_STR, stripped.display())
                } else {
                    path.display().to_string()
                }
            }).collect()
        };

        println!(
            "{}{}{}",
            if args.display_count {
                format!("{}{}", count, &args.separator)
            } else {
                String::new()
            },
            path_strings.join(&args.separator),
            &args.group_separator,
        );
    }

    if errors.len() > 0 {
        eprintln!("{}", format!("{} ERRORS {}", "=".repeat(30), "=".repeat(30)).color(Color::BrightRed));
        for error in errors {
            eprintln!("{}", error);
        }
        eprintln!("{}", String::from("Results may be invalid due to the above errors.").color(Color::BrightRed))
    }
}
