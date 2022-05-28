# detect-duplicates
Detects and reports duplicate files in a file system.

This program runs in linear time with respect to the number of files being checked and loads (in expectation) only two files into memory at a time, and can thus be used to efficiently check huge amounts of very large files for duplicates.

## Installation:
You will need to install [Rust](https://www.rust-lang.org/learn/get-started) in order to build this program from source. 
After installing Rust, installation is simply:
```
git clone https://github.com/kpberry/duplicate-detector
cd duplicate-detector
cargo install --path .
```

## Usage:
This program takes a path to a directory and outputs any files which are copies of another file within that directory or any of its children. Each group of equal files will be output as a comma separated list.

For example, given the following directory structure:
```
files
├── a.txt
├── b.txt
└── more_files
    ├── c.txt
    ├── d.txt
    └── even_more_files
        ├── e.txt
        └── f.txt
```
If `a.txt` and `e.txt` have the same contents and `b.txt`, `c.txt`, and `d.txt` have the same contents, then this command:
```
detect-duplicates files
```
will produce the following output:
```
files/a.txt, files/more_files/even_more_files/e.txt
files/b.txt, files/more_files/c.txt, files/more_files/d.txt
```

And this command:
```
detect-duplicates files/more_files
```
would output
```
files/more_files/c.txt, files/more_files/d.txt
```

## Technical details
Comparing large files is expensive, so we want to minimize the amount of comparisons done. Reading files is also expensive, but not as expensive as buying more RAM. We can achieve a linear number of comparisons while reading each file at most twice by doing the following:
1. We compute a hash of the contents of each file, and store a mapping from the **hashes** to lists of files with that same hash.
2. For each list of files with at least two elements with the same hash, we store a mapping from the **contents** to the lists of files with the same contents.
3. We report as duplicates all files which have the same key in the second map.

Because we use high quality 64 bit hashes, the probability of more than one file having the same content hash but different contents is roughly #files with same content hash/2^64, which should be extremely small in most practical cases. Thus, we expect there to only be one key in the second map at a time and one file loaded, which we need to compare to the value in the map, meaning that, at most, we expect to have two files loaded simultaneously into RAM. Each file is loaded at most twice in total; once to compute the key for the first map, and once to compute the key in the second map, if applicable. 