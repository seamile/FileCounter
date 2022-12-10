use clap::Parser;
use std::{env, fs, os::macos::fs::MetadataExt, path::PathBuf, process::exit};

const SZ_UNIT: [&str; 7] = ["B", "K", "M", "G", "T", "P", "E"];

#[derive(Parser)]
#[command(name = "fcnt")]
#[command(version = "0.1.0")]
#[command(about = "Count the total number of files in given directories.")]
struct CmdLineArgs {
    /// the directories (default: ./)
    directories: Vec<String>,

    /// count all regular and hidden files.
    #[arg(short = 'a')]
    all_files: bool,

    /// count the number of directories.
    #[arg(short = 'd')]
    count_dirs: bool,

    /// count the total size of files.
    #[arg(short = 's')]
    count_size: bool,
}

impl CmdLineArgs {
    fn get_directories(&self) -> Vec<PathBuf> {
        let mut directories: Vec<PathBuf> = Vec::new();
        if self.directories.is_empty() {
            directories.push(env::current_dir().unwrap());
        } else {
            for dir in self.directories.iter().map(|p| PathBuf::from(p)) {
                if dir.is_dir() {
                    directories.push(dir);
                } else {
                    println!("wrong arg: {:?} is not directory.", dir);
                }
            }
            if directories.is_empty() {
                println!("error: non directories.");
                exit(1);
            }
        }
        return directories;
    }
}

#[allow(unused)]
pub struct Counter {
    pub n_files: u64,
    pub n_dirs: u64,
    pub size: u64,
}

impl Counter {
    pub fn new() -> Counter {
        return Counter {
            n_files: 0,
            n_dirs: 0,
            size: 0,
        };
    }

    // the total block size used by a file
    pub fn file_size(metadata: &fs::Metadata) -> u64 {
        let sz = metadata.st_size() as f64;
        let blksz = metadata.st_blksize() as f64;
        return (blksz * (sz / blksz).ceil()) as u64;
    }

    pub fn readable_size(size: u64) -> String {
        let mut sz = size as f64;
        let mut str_sz = String::new();

        for unit in SZ_UNIT {
            if sz >= 1024.0 {
                sz = sz / 1024.0;
            } else {
                if sz.fract() < 0.1 {
                    str_sz = format!("{:.0}{}", sz, unit);
                } else {
                    str_sz = format!("{:.1}{}", sz, unit);
                }
                break;
            }
        }
        return str_sz;
    }

    pub fn walk_dir(&mut self, dirpath: PathBuf) {
        for entry in Walker::new(&dirpath) {
            let path = entry.path();
            if path.is_dir() {
                self.n_dirs += 1;
            }
        }
    }
}

// walk all the files in dir
struct Walker {
    readers: Vec<fs::ReadDir>,
}

impl Walker {
    pub fn new(dirpath: &PathBuf) -> Walker {
        if dirpath.is_dir() {
            let rd = fs::read_dir(dirpath).unwrap();
            return Walker { readers: vec![rd] };
        } else {
            panic!("the {:?} is not a directory.", dirpath);
        }
    }
}

impl Iterator for Walker {
    type Item = fs::DirEntry;

    fn next(&mut self) -> Option<fs::DirEntry> {
        if self.readers.is_empty() {
            return None;
        } else {
            let reader = &mut self.readers[0];
            if let Some(entry) = reader.next() {
                let entry: fs::DirEntry = entry.unwrap();
                if entry.path().is_dir() {
                    let sub_reader = fs::read_dir(&entry.path()).unwrap();
                    self.readers.push(sub_reader);
                }
                return Some(entry);
            } else {
                self.readers.remove(0);
                return self.next();
            }
        }
    }
}

fn main() {
    // parse cmd-line args and get directories
    let args = CmdLineArgs::parse();
    let directories = args.get_directories();

    // walk all files in the directories
    for dir in directories {
        for entry in Walker::new(&dir) {
            if let Ok(_meta) = entry.metadata() {
                if let Some(name) = entry.path().file_name() {
                    println!("{:?}: ", name);
                }
            }
        }
    }
}

#[test]
fn test_readable_size() {
    assert_eq!(Counter::readable_size(1023), "1023B");
    assert_eq!(Counter::readable_size(1434), "1.4K");
    assert_eq!(Counter::readable_size(15926), "15.6K");
    assert_eq!(Counter::readable_size(53589793), "51.1M");
    assert_eq!(Counter::readable_size(238462643383), "222G");
    assert_eq!(Counter::readable_size(279502884197169), "254.2T");
    assert_eq!(Counter::readable_size(0xffffffffffffffff), "16E");
}
