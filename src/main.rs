use clap::Parser;
use std::{env::current_dir, fs, io, os::macos::fs::MetadataExt, path::PathBuf};

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

// the total block size used by a file
fn file_size(metadata: &fs::Metadata) -> u64 {
    let sz = metadata.st_size() as f64;
    let blksz = metadata.st_blksize() as f64;
    return (blksz * (sz / blksz).ceil()) as u64;
}

// walk all the files in dir
fn walk_dir(dir_path: &PathBuf) -> io::Result<()> {
    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let name = entry.file_name();
                let size = file_size(&entry.metadata()?);
                println!("{:15} sz={:<5}B", name.to_str().unwrap(), size);
            }
        }
    }
    Ok(())
}

fn main() {
    let args = CmdLineArgs::parse();

    // get the directories
    let mut directories: Vec<PathBuf> = Vec::new();
    if args.directories.is_empty() {
        directories.push(current_dir().unwrap());
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
        let _ = walk_dir(&dir);
    }

    println!("all_files: {:?}", args.all_files);
    println!("count_dirs: {:?}", args.count_dirs);
    println!("count_size: {:?}", args.count_size);
}
