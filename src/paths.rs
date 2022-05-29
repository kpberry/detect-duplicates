//! Convenience functions for working with paths.

use std::{
    fs,
    path::{Path, PathBuf},
};

/// Recursively finds all of the descendants of a file.
/// 
/// ## Example:
/// ```no_run
/// let paths = vec![
///     Path::from("files/a.txt"), 
///     Path::from("files/b.txt"), 
///     Path::from("files/more_files/c.txt"),
///     Path::from("files/more_files/d.txt"),
///     Path::from("files/more_files/even_more_files/e.txt"),
///     Path::from("files/more_files/even_more_files/f.txt")
/// ];
/// 
/// let descendants = get_descendants(Path::from("files/more_files"));
/// let expected = vec![
///     Path::from("files/more_files/c.txt"),
///     Path::from("files/more_files/d.txt"),
///     Path::from("files/more_files/even_more_files/e.txt"),
///     Path::from("files/more_files/even_more_files/f.txt")
/// ];
/// assert!(descendants == expected);
/// 
/// let descendants = get_descendants(Path::from("files"));
/// assert!(descendants == paths);
/// ```
pub fn get_descendants(base_path: &Path) -> Vec<PathBuf> {
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