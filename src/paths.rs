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
pub fn get_descendants(base_path: &Path, max_depth: Option<usize>) -> Vec<PathBuf> {
    if let Some(max_depth) = max_depth {
        if max_depth == 0 {
            return Vec::new();
        }
    }

    if base_path.is_dir() {
        if let Ok(dir_iter) = fs::read_dir(base_path) {
            dir_iter
                .flatten()
                .map(|dir_entry| get_descendants(&dir_entry.path(), max_depth.map(|d| d - 1)))
                .flatten()
                .collect()
        } else {
            Vec::new()
        }
    } else {
        vec![PathBuf::from(base_path)]
    }
}

pub fn get_common_prefix(paths: &[PathBuf]) -> PathBuf {
    if paths.len() == 0 {
        return PathBuf::new();
    }

    let base_path = &paths[0];
    let mut last_match = base_path.components().collect::<Vec<_>>().len();
    for path in paths.iter().skip(1) {
        for (i, (a, b)) in base_path.components().zip(path.components()).enumerate() {
            if i >= last_match || a != b {
                last_match = i;
                break;
            }
        }
    }

    base_path.components().take(last_match).collect()
}
