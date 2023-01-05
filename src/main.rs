mod output;
mod walker;

use clap::Parser;
use std::path::PathBuf;
use std::process::exit;

use walker::Counter;

fn main() {
    // parse cmd-line args and get directories
    let args = CmdLineArgs::parse();

    // walk all files
    let directories = args.get_directories();
    let mut counters = Vec::<Counter>::new();
    if args.non_recursive {
        for dirpath in directories {
            if let Ok((_, counter)) = walker::walk(&dirpath, args.all_files, args.count_size) {
                counters.push(counter);
            };
        }
    } else {
        counters = walker::parallel_walk(directories, args.all_files, args.count_size);
    }
    Counter::output(&counters, args.count_size);
}

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

    /// count the total size of files.
    #[arg(short = 's')]
    count_size: bool,

    /// non-recursive mode (files in sub-directories will be ignored).
    #[arg(short = 'R')]
    non_recursive: bool,
}

impl CmdLineArgs {
    fn get_directories(&self) -> Vec<PathBuf> {
        let mut directories: Vec<PathBuf> = vec![];
        if self.directories.is_empty() {
            directories.push(PathBuf::from("./"));
        } else {
            for dir in self.directories.iter().map(|p| PathBuf::from(p)) {
                if dir.is_dir() {
                    directories.push(dir);
                } else {
                    let msg = format!("fcnt: {:?} is not directory.", dir);
                    println!("{}", output::err(&msg));
                }
            }
            if directories.is_empty() {
                println!("{}", output::err(&"fcnt: non directories."));
                exit(1);
            }
        }
        return directories;
    }
}
