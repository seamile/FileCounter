use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
use num_cpus;

use crate::output as op;

#[derive(Parser)]
#[command(name = "fcnt")]
#[command(version = "0.2.2")]
#[command(about = "Count the total number of files in given directories.")]
pub struct CmdArgParser {
    /// the directories (default: ./)
    pub directories: Vec<String>,

    /// count all regular and hidden files.
    #[arg(short = 'a')]
    pub all_files: bool,

    /// count the total size of files.
    #[arg(short = 's')]
    pub with_size: bool,

    /// non-recursive mode (files in sub-directories will be ignored).
    #[arg(short = 'R')]
    pub non_recursive: bool,

    /// the number of threads for traversal (invalid in `non_recursive` mode).
    #[arg(short = 't')]
    pub n_thread: Option<usize>,
}

impl CmdArgParser {
    pub fn get_threads_num(&self) -> usize {
        match self.n_thread {
            Some(num) => return num,
            None => {
                let n_cpu = num_cpus::get();
                return if n_cpu >= 4 { n_cpu } else { 4 };
            }
        }
    }

    pub fn get_directories(&self) -> Vec<PathBuf> {
        let mut directories: Vec<PathBuf> = vec![];
        if self.directories.is_empty() {
            directories.push(PathBuf::from("./"));
        } else {
            for dir in self.directories.iter().map(|p| PathBuf::from(p)) {
                if dir.is_dir() {
                    directories.push(dir);
                } else {
                    let msg = format!("{:?} is not a directory.", dir);
                    println!("{}", op::warn(&msg));
                }
            }
            if directories.is_empty() {
                println!("{}", op::err(&"fcnt: no directory found."));
                exit(1);
            }
        }
        return directories;
    }
}
