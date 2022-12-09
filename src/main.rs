use clap::Parser;
use std::{env, fs, os::macos::fs::MetadataExt, path::PathBuf};

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
                    return self.next();
                } else {
                    return Some(entry);
                }
            } else {
                self.readers.remove(0);
                return self.next();
            }
        }
    }
}

// the total block size used by a file
fn file_size(metadata: &fs::Metadata) -> u64 {
    let sz = metadata.st_size() as f64;
    let blksz = metadata.st_blksize() as f64;
    return (blksz * (sz / blksz).ceil()) as u64;
}

fn main() {
    let args = CmdLineArgs::parse();

    // get the directories
    let mut directories: Vec<PathBuf> = Vec::new();
    if args.directories.is_empty() {
        directories.push(env::current_dir().unwrap());
    } else {
        for dir in args.directories.iter().map(|p| PathBuf::from(p)) {
            if dir.is_dir() {
                directories.push(dir);
            } else {
                println!("{:?} is not directory.", dir);
            }
        }
    }

    for dir in directories {
        let walker = Walker::new(&dir);
        for entry in walker {
            if let Ok(meta) = entry.metadata() {
                if let Some(name) = entry.path().file_name() {
                    println!("{:?}: {:?}", name, file_size(&meta));
                }
            }
        }
    }

    println!("all_files: {:?}", args.all_files);
    println!("count_dirs: {:?}", args.count_dirs);
    println!("count_size: {:?}", args.count_size);
}
