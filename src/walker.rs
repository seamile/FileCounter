use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;
#[cfg(target_os = "macos")]
use std::os::macos::fs::MetadataExt;
#[cfg(target_os = "unix")]
use std::os::unix::fs::MetadataExt;

type DirList = Vec<PathBuf>;
type SizeMap = HashMap<u64, u64>;
pub type DirDetail = (DirList, Counter);

pub struct Counter {
    pub dirpath: PathBuf,
    pub n_files: u64,
    pub n_dirs: u64,
    pub sz_map: SizeMap,
}

#[allow(unused)]
impl Counter {
    const SZ_UNIT: [&str; 7] = ["B", "K", "M", "G", "T", "P", "E"];

    pub fn new(dirpath: &PathBuf) -> Counter {
        return Counter {
            dirpath: dirpath.clone(),
            n_files: 0,
            n_dirs: 0,
            sz_map: SizeMap::new(),
        };
    }

    pub fn name(&self) -> Cow<str> {
        if let Some(dirname) = self.dirpath.file_name() {
            return dirname.to_string_lossy();
        } else if let Some(dirname) = self.dirpath.to_str() {
            return Cow::from(dirname);
        } else {
            return Cow::from("-");
        }
    }

    fn file_size(metadata: &fs::Metadata) -> u64 {
        let sz = metadata.st_size() as f64;
        let blksz = metadata.st_blksize() as f64;
        return (blksz * (sz / blksz).ceil()) as u64;
    }

    pub fn total_size(&self) -> u64 {
        self.sz_map.values().sum()
    }

    // Make "size" more readable
    pub fn readable_size(&self) -> String {
        let mut sz = self.total_size() as f64;
        let mut str_sz = String::new();

        for unit in Counter::SZ_UNIT {
            if sz >= 1024.0 {
                sz = sz / 1024.0;
            } else {
                if sz.fract() < 0.05 {
                    str_sz = format!("{:.0}{}", sz, unit);
                } else {
                    str_sz = format!("{:.1}{}", sz, unit);
                }
                break;
            }
        }
        return str_sz;
    }

    // update from anther Counter of sub-dir
    pub fn update(&mut self, other: Counter) {
        if other.dirpath.starts_with(&self.dirpath) {
            self.n_files += other.n_files;
            self.n_dirs += other.n_dirs;
            self.sz_map.extend(other.sz_map);
        }
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "{:10}: {:5} files | {:5} dirs | {}",
            self.name(),
            self.n_files,
            self.n_dirs,
            self.readable_size()
        );
    }
}

pub fn walk(dirpath: &PathBuf, ignore_hidden: bool, count_sz: bool) -> Result<DirDetail> {
    let mut dirs = DirList::new();
    let mut cnt = Counter::new(dirpath);

    for entry in fs::read_dir(dirpath)? {
        let entry = entry?;
        let path = entry.path();
        let fname = entry.file_name();

        if ignore_hidden && fname.to_string_lossy().starts_with('.') {
            // ignore the hidden files and dirs
            continue;
        } else if path.is_symlink() {
            // The size of symbolic link is 0B.
            // So just increase the num of files here.
            cnt.n_files += 1;
        } else if path.is_dir() {
            cnt.n_dirs += 1;
            dirs.push(path);
        } else {
            cnt.n_files += 1;
            if count_sz {
                // count file size and insert into SizeMap
                let meta = entry.metadata()?;
                cnt.sz_map.insert(meta.st_ino(), Counter::file_size(&meta));
            }
        }
    }

    return Ok((dirs, cnt));
}

#[test]
fn test_walk() {
    // walk(&PathBuf::from("/Users/xu/src/"), false);
}

#[test]
fn test_readable_size() {
    let mut c = Counter::new(&PathBuf::from("."));

    c.sz_map.insert(1, 1023);
    assert_eq!(c.readable_size(), "1023B");

    c.sz_map.insert(1, 1434);
    assert_eq!(c.readable_size(), "1.4K");

    c.sz_map.insert(1, 15926);
    assert_eq!(c.readable_size(), "15.6K");

    c.sz_map.insert(1, 53589793);
    assert_eq!(c.readable_size(), "51.1M");

    c.sz_map.insert(1, 238462643383);
    assert_eq!(c.readable_size(), "222.1G");

    c.sz_map.insert(1, 279502884197169);
    assert_eq!(c.readable_size(), "254.2T");

    c.sz_map.insert(1, 0xffffffffffffffff);
    assert_eq!(c.readable_size(), "16E");
}
