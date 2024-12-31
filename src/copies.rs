//! Functions for counting copies in a set of paths.

use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs,
    hash::{Hash, Hasher},
    path::PathBuf,
};

/// Returns a list of all sets of copies of files in a set of paths.
///
/// Each entry in the output list will be a list of files from the input paths
/// which have the same contents. The order of the output is nondeterministic.
///
/// Runs in O(NF) time and O(nF) memory, where N is the total number of files,
/// n is the number of unique files, and F is the average file size.
///
/// ## Note
/// If you are loading more than a few files, prefer to use
/// [`get_copies_hashed`], since it uses much less memory. This is
/// intented to be used as a subroutine when the expected number of unique
/// files is small.
///
/// ## Example
/// ```no_run
/// // assume that a.txt and e.txt have the same contents, and b.txt, c.txt and d.txt have the same contents
/// let paths = vec![
///     Path::from("files/a.txt"),
///     Path::from("files/b.txt"),
///     Path::from("files/more_files/c.txt"),
///     Path::from("files/more_files/d.txt"),
///     Path::from("files/more_files/even_more_files/e.txt"),
///     Path::from("files/more_files/even_more_files/f.txt")
/// ];
/// let copies = get_copies(&paths);
/// let expected = vec![
///     vec!["files/a.txt", "files/more_files/even_more_files/e.txt"],
///     vec!["files/b.txt", "files/more_files/c.txt", "files/more_files.d.txt"]
/// ];
/// assert!(copies == expected);
/// ```
pub fn get_copies(paths: &[PathBuf]) -> Vec<Vec<PathBuf>> {
    let mut copies = HashMap::new();
    for path in paths {
        let contents = fs::read(&path);
        if let Ok(contents) = contents {
            copies.entry(contents).or_insert(vec![]).push(path.clone());
        }
    }
    copies.values().cloned().collect()
}

/// Returns a list of all sets of copies of files in a set of paths in a memory efficient manner.
///
/// Each entry in the output list will be a list of files from the input paths
/// which have the same contents. The order of the output is nondeterministic.
///
/// Runs in O(NF) time and O(n) memory in expectation, where N is the total
/// number of files, n is the number of unique files, and F is the average
/// file size.
///
/// ## Example
/// ```no_run
/// // assume that a.txt and e.txt have the same contents, and b.txt, c.txt and d.txt have the same contents
/// let paths = vec![
///     Path::from("files/a.txt"),
///     Path::from("files/b.txt"),
///     Path::from("files/more_files/c.txt"),
///     Path::from("files/more_files/d.txt"),
///     Path::from("files/more_files/even_more_files/e.txt"),
///     Path::from("files/more_files/even_more_files/f.txt")
/// ];
/// let copies = get_copies(&paths);
/// let expected = vec![
///     vec!["files/a.txt", "files/more_files/even_more_files/e.txt"],
///     vec!["files/b.txt", "files/more_files/c.txt", "files/more_files.d.txt"]
/// ];
/// assert!(copies == expected);
/// ```
pub fn get_copies_hashed(paths: &[PathBuf]) -> Vec<Vec<PathBuf>> {
    let mut candidate_copies: HashMap<(u64, usize), Vec<PathBuf>> = HashMap::new();
    for path in paths.iter().cloned() {
        let contents = fs::read(&path);
        if let Ok(contents) = contents {
            let mut hasher = DefaultHasher::new();
            contents.hash(&mut hasher);
            let contents_hash = hasher.finish();
            let key = (contents_hash, contents.len());
            candidate_copies.entry(key).or_insert(vec![]).push(path);
        }
    }

    candidate_copies
        .values()
        .map(|candidates| get_copies(candidates))
        .flatten()
        .collect()
}
