//! Functions for counting copies in a set of paths.

use core::fmt;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Seek},
    path::{Path, PathBuf},
};

/// Struct which wraps a std::io::Error to include the path for which the error occurred.
pub struct PathIoError {
    error: std::io::Error,
    path: PathBuf,
}

impl fmt::Display for PathIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.path.display(),
            self.error,
        )
    }
}

/// Returns a list of all sets of copies of files in a set of paths.
///
/// Each entry in the output list will be a list of files from the input paths
/// which have the same contents. The order of the output is not deterministic.
///
/// Runs in O(NF) time and O(nF) memory, where N is the total number of files,
/// n is the number of unique files, and F is the average file size.
/// 
/// # Errors
/// 
/// Any errors reading files will be included in the second index of the
/// return value.
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
/// let (copies, errors) = get_copies(&paths);
/// let expected = vec![
///     vec!["files/a.txt", "files/more_files/even_more_files/e.txt"],
///     vec!["files/b.txt", "files/more_files/c.txt", "files/more_files.d.txt"]
/// ];
/// assert!(copies == expected);
/// assert!(errors.len() == 0);
/// ```
pub fn get_copies(paths: &[PathBuf]) -> (Vec<Vec<PathBuf>>, Vec<PathIoError>) {
    if paths.len() == 1 {
        return (vec![vec![paths[0].clone()]], vec![]);
    }

    // We can skip the hashing in this case, which saves time
    if paths.len() == 2 {
        let (a, b) = (paths[0].clone(), paths[1].clone());

        return match (fs::read(&a), fs::read(&b)) {
            (Ok(a_contents), Ok(b_contents)) => {
                if a_contents == b_contents {
                    (vec![vec![a, b]], vec![])
                } else {
                    (vec![vec![a], vec![b]], vec![])
                }
            }
            (Ok(_), Err(err)) => (
                vec![vec![a]],
                vec![PathIoError {
                    error: err,
                    path: b,
                }],
            ),
            (Err(err), Ok(_)) => (
                vec![vec![b]],
                vec![PathIoError {
                    error: err,
                    path: a,
                }],
            ),
            (Err(err_a), Err(err_b)) => (
                vec![],
                vec![
                    PathIoError {
                        error: err_a,
                        path: a,
                    },
                    PathIoError {
                        error: err_b,
                        path: b,
                    },
                ],
            ),
        };
    }

    let mut copies = HashMap::new();
    let mut errors = Vec::new();
    for path in paths {
        match fs::read(&path) {
            Ok(contents) => {
                copies.entry(contents).or_insert(vec![]).push(path.clone());
            },
            Err(err) => {
                errors.push(PathIoError { error: err, path: path.to_path_buf()})
            },
        }
    }
    (copies.values().cloned().collect(), errors)
}

/// Returns a "fingerprint" for a file which should uniquely identify its contents with high probability.
///
/// This runs in constant time; for small files (under one page in length),
/// the entire file is read, and we concatenate first byte of each eighth of
/// the file to build the fingerprint. For larger files, we seek each eighth
/// of the file instead, again taking the first byte of each. Additionally,
/// we include the length of the file in the identifier.
/// 
/// # Errors
///
/// This function will return an error if the file cannot be opened or read,
/// or if a seek fails while iterating over the file contents.
pub fn get_fingerprint(path: &Path) -> Result<(usize, usize), std::io::Error> {
    let mut file = File::open(&path)?;
    let len = file.metadata().map_or(0, |md| md.len()) as usize;

    if len == 0 {
        return Ok((0, 0));
    }

    let mut contents: Vec<u8> = Vec::with_capacity(len.min(64));
    if len <= 4096 {
        let bytes = fs::read(&path)?;
        for i in (0..len).step_by((len / 8).max(1)) {
            contents.push(bytes[i]);
        }
    } else {
        let steps = 4;
        let read_size = 8 / steps;
        let step = (len / steps) as i64;
        let mut buf: Vec<u8> = (0..read_size).map(|_| 0).collect();
        for _ in 0..steps {
            file.read_exact(&mut buf)?;
            for &b in buf.iter() {
                contents.push(b);
            }
            file.seek(std::io::SeekFrom::Current(step))?;
        }
    }

    let mut key = 0;
    for (i, &c) in contents.iter().enumerate() {
        key |= (c as usize) << (i << 3);
    }

    Ok((key, len))
}

/// Returns a list of all sets of copies of files in a set of paths in a memory efficient manner.
/// 
/// Each entry in the output list will be a list of files from the input paths
/// which have the same contents. The order of the output is not deterministic.
///
/// Runs in O(NF) time and O(n) memory in expectation, where N is the total
/// number of files, n is the number of unique files, and F is the average
/// file size.
/// 
/// # Errors
/// 
/// Any errors reading files will be included in the second index of the
/// return value.
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
/// let (copies, errors) = get_copies(&paths);
/// let expected = vec![
///     vec!["files/a.txt", "files/more_files/even_more_files/e.txt"],
///     vec!["files/b.txt", "files/more_files/c.txt", "files/more_files.d.txt"]
/// ];
/// assert!(copies == expected);
/// assert!(errors.len() == 0);
/// ```
pub fn get_copies_hashed(paths: &[PathBuf]) -> (Vec<Vec<PathBuf>>, Vec<PathIoError>) {
    let mut candidate_copies = HashMap::new();
    let mut errors: Vec<PathIoError> = Vec::new();

    for path in paths {
        match get_fingerprint(path) {
            Ok(fingerprint) => {
                candidate_copies
                    .entry(fingerprint)
                    .or_insert(vec![])
                    .push(path.clone());
            }
            Err(err) => errors.push(PathIoError {
                error: err,
                path: path.to_path_buf(),
            }),
        }
    }

    let mut copies: Vec<Vec<PathBuf>> = Vec::new();
    for group in candidate_copies.values() {
        let (group_copies, group_errors) = get_copies(group);
        copies.extend(group_copies);
        errors.extend(group_errors);
    }
    
    (copies, errors)
}
