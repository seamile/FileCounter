use std::path::{PathBuf, MAIN_SEPARATOR};
use std::process::exit;

use clap::{Parser, ValueEnum};
use num_cpus;
use regex::Regex;

use crate::output::err;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OrderBy {
    /// order by pathname
    Name,
    /// alias of Name
    N,
    /// order by number of files
    File,
    /// alias of File
    F,
    /// order by number of directories
    Dir,
    /// alias of Dir
    D,
    /// order by size of each directory
    Size,
    /// alias of Size
    S,
}

#[derive(Parser)]
#[command(name = "fcnt")]
#[command(version = "0.2.6")]
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

    /// The value to sort the results by.
    #[arg(short = 'o', value_enum)]
    pub order_by: Option<OrderBy>,

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
        if self.directories.is_empty() {
            vec![PathBuf::from(".")]
        } else {
            self.directories
                .iter()
                .map(|p| {
                    let len = p.len();
                    if len > 1 {
                        PathBuf::from(p.trim_end_matches(MAIN_SEPARATOR))
                    } else {
                        PathBuf::from(p)
                    }
                })
                .filter(|p| p.is_dir())
                .collect()
        }
    }
}
