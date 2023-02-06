use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
use num_cpus;
use regex::Regex;

use crate::output::{err, warn};

#[derive(Parser)]
#[command(name = "fcnt")]
#[command(version = "0.2.4")]
#[command(about = "Count the total number of files in given directories.")]
pub struct CmdArgParser {
    /// The directories (default: ./).
    pub directories: Vec<String>,

    /// Count all regular and hidden files.
    #[arg(short = 'a')]
    pub all_files: bool,

    /// Match entries using regex (only matche filenames).
    #[arg(short = 'r', value_name = "PATTERN")]
    pub re: Option<String>,

    /// Count the total size of files.
    #[arg(short = 's')]
    pub with_size: bool,

    /// The number of threads for traversal (invalid in `non_recursive` mode).
    #[arg(short = 't', value_name = "THREAD_NUM")]
    pub n_thread: Option<usize>,

    /// Verbose mode, open this option will display the found entries.
    #[arg(short = 'v')]
    pub verbose: bool,

    /// Non-recursive mode (files in sub-directories will be ignored).
    #[arg(short = 'R')]
    pub non_recursive: bool,
}

impl CmdArgParser {
    pub fn get_regex(&self) -> Option<Regex> {
        if let Some(re) = &self.re {
            if let Ok(filter) = Regex::new(re.as_str()) {
                return Some(filter);
            } else {
                println!("{}", err(&format!("Involid regex pattern: {}", re)));
                exit(1);
            }
        } else {
            return None;
        }
    }

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
                    println!("{}", warn(&msg));
                }
            }
            if directories.is_empty() {
                println!("{}", err(&"Fcnt: no directory found."));
                exit(1);
            }
        }
        return directories;
    }
}
