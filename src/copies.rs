//! Functions for counting copies in a set of paths.

use std::{
    collections::HashMap, fs::{self, File}, io::{Read, Seek}, path::PathBuf
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
    let mut candidate_copies: HashMap<(Vec<u8>, usize), Vec<PathBuf>> = HashMap::new();
    for path in paths.iter().cloned() {
        let file = File::open(&path);
        match file {
            Ok(mut file) => {
                let len = file.metadata().map_or(0, |md| md.len()) as usize;
                let contents = if len <= 4096 {
                    fs::read(&path).unwrap()
                } else {
                    let steps = 4;
                    let read_size = 64 / steps;
                    let step = (len / steps) as i64;
                    let mut buf: Vec<u8> = (0..read_size).map(|_| 0).collect();
                    let mut contents_buf = Vec::with_capacity(64);
                    for _ in 0..steps {
                        file.read_exact(&mut buf).unwrap();
                        contents_buf.push(buf[0]);
                        file.seek(std::io::SeekFrom::Current(step)).unwrap();
                    }
                    buf
                };

                let key = (contents, len);
                candidate_copies.entry(key).or_insert(vec![]).push(path);
            },
            Err(err) => eprintln!("error opening file {}: {}", path.display().to_string(), err),
        }
    }

    candidate_copies
        .values()
        .map(|candidates| get_copies(candidates))
        .flatten()
        .collect()
}
