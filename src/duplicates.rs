use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs,
    hash::{Hash, Hasher},
    path::PathBuf,
};

pub fn get_duplicates(paths: &Vec<PathBuf>) -> Vec<Vec<PathBuf>> {
    let mut duplicates = HashMap::new();
    for path in paths {
        let contents = fs::read(&path);
        if let Ok(contents) = contents {
            duplicates
                .entry(contents)
                .or_insert(vec![])
                .push(path.clone());
        }
    }
    duplicates
        .values()
        .filter(|candidates| candidates.len() > 1)
        .cloned()
        .collect()
}

pub fn get_duplicates_hashed(paths: &Vec<PathBuf>) -> Vec<Vec<PathBuf>> {
    let mut candidate_duplicates: HashMap<(u64, usize), Vec<PathBuf>> = HashMap::new();
    for path in paths.iter().cloned() {
        let contents = fs::read(&path);
        if let Ok(contents) = contents {
            let mut hasher = DefaultHasher::new();
            contents.hash(&mut hasher);
            let contents_hash = hasher.finish();
            let key = (contents_hash, contents.len());
            candidate_duplicates.entry(key).or_insert(vec![]).push(path);
        }
    }

    candidate_duplicates
        .values()
        .filter(|candidates| candidates.len() > 1)
        .map(|candidates| get_duplicates(candidates))
        .flatten()
        .collect()
}
