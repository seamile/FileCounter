use std::collections::HashMap;
use std::fs;
use std::io::Result;
#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;
#[cfg(target_os = "macos")]
use std::os::macos::fs::MetadataExt;
#[cfg(target_os = "unix")]
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::sync::mpsc::channel as s_channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use flume::unbounded as m_channel;

use crate::output as op;

type DirList = Vec<PathBuf>;
type SizeMap = HashMap<u64, u64>;
type DirDetail = (DirList, Counter);
type Lengths = (usize, usize, usize, usize);

#[derive(Debug)]
pub struct Counter {
    pub dirpath: PathBuf,
    pub n_files: u64,
    pub n_dirs: u64,
    pub sz_map: Option<SizeMap>,
}

impl Counter {
    const SZ_UNIT: [&str; 7] = ["B", "K", "M", "G", "T", "P", "E"];

    /// Create a new Counter
    pub fn new(dirpath: &PathBuf, with_size: bool) -> Self {
        return Self {
            dirpath: dirpath.clone(),
            n_files: 0,
            n_dirs: 0,

            sz_map: if with_size {
                Some(SizeMap::new())
            } else {
                None
            },
        };
    }

    // the dirpath with `&str` type
    fn path(&self) -> &str {
        return self.dirpath.to_str().expect("dir path err");
    }

    // get the file size from Metadata
    fn file_size(metadata: &fs::Metadata) -> u64 {
        let sz = metadata.st_size() as f64;
        let blksz = metadata.st_blksize() as f64;
        return (blksz * (sz / blksz).ceil()) as u64;
    }

    // calculate the total size of files in dirpath
    pub fn size(&self) -> u64 {
        match self.sz_map.as_ref() {
            Some(mp) => mp.values().sum(),
            None => 0,
        }
    }

    fn add_unit_to_size(size: u64) -> String {
        let mut sz = size as f64;
        let mut str_sz = String::new();

        for unit in Self::SZ_UNIT {
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

    // make "size" more readable
    fn readable_size(&self) -> String {
        return Self::add_unit_to_size(self.size());
    }

    // merge from anther Counter
    fn merge(&mut self, other: &Self) {
        if other.dirpath.starts_with(&self.dirpath) {
            self.n_files += other.n_files;
            self.n_dirs += other.n_dirs;
            if let Some(sz_mp) = self.sz_map.as_mut() {
                sz_mp.extend(other.sz_map.as_ref().unwrap().iter());
            }
        }
    }

    // get the length of each field for display
    fn lengths(&self) -> Lengths {
        return (
            op::display_width(&self.path().to_string()),
            self.n_files.to_string().len(),
            self.n_dirs.to_string().len(),
            self.readable_size().len(),
        );
    }

    fn join_fields(fields: Vec<&dyn ToString>, with_size: bool, lens: Lengths) -> String {
        let f0 = op::left_justify(fields[0], lens.0);
        let f1 = op::right_justify(fields[1], lens.1);
        let f2 = op::right_justify(fields[2], lens.2);
        if with_size {
            let f3 = op::right_justify(fields[3], lens.3);
            return vec![f0, f1, f2, f3].join("  ");
        } else {
            return vec![f0, f1, f2].join("  ");
        }
    }

    fn to_string(&self, lens: Lengths) -> String {
        let path = self.path();
        let size = self.readable_size();
        let fields: Vec<&dyn ToString> = vec![&path, &self.n_files, &self.n_dirs, &size];
        let with_size = self.sz_map != None;
        return Self::join_fields(fields, with_size, lens);
    }

    fn make_head_line(with_size: bool, lens: Lengths) -> String {
        let fields: Vec<&dyn ToString> = vec![&"Path", &"Files", &"Dirs", &"Size"];
        return op::title(&Self::join_fields(fields, with_size, lens));
    }

    fn make_total_line(
        total: (String, String, String, String),
        with_size: bool,
        lens: Lengths,
    ) -> String {
        let fields: Vec<&dyn ToString> = vec![&total.0, &total.1, &total.2, &total.3];
        let total_line = Self::join_fields(fields, with_size, lens);
        let hor_line = op::fill_char('─', total_line.len());

        return format!("{}\n{}", hor_line, op::strong(&total_line));
    }

    fn summarize(counters: &Vec<Self>) -> (String, String, String, String) {
        let mut sum = (0_u64, 0_u64, 0_u64);
        for c in counters {
            sum.0 += c.n_files;
            sum.1 += c.n_dirs;
            sum.2 += c.size();
        }

        return (
            String::from("Total"),
            sum.0.to_string(),
            sum.1.to_string(),
            Self::add_unit_to_size(sum.2),
        );
    }

    fn max_lengths(lens: Vec<Lengths>) -> Lengths {
        let mut max_lens: Lengths = (0, 0, 0, 0);
        for (l0, l1, l2, l3) in lens {
            max_lens.0 = max_lens.0.max(l0);
            max_lens.1 = max_lens.1.max(l1);
            max_lens.2 = max_lens.2.max(l2);
            max_lens.3 = max_lens.3.max(l3);
        }
        return max_lens;
    }

    pub fn output(counters: &Vec<Self>, with_size: bool) {
        let total = if counters.len() > 1 {
            Self::summarize(counters)
        } else {
            (String::new(), String::new(), String::new(), String::new())
        };

        // calculate the max value from `title`, `total` and `contents` lengths
        let title_lens = (4_usize, 5_usize, 4_usize, 4_usize);
        let total_lens = (total.0.len(), total.1.len(), total.2.len(), total.3.len());
        let mut lens = Vec::from_iter(counters.iter().map(|c| c.lengths()));
        lens.append(&mut vec![total_lens, title_lens]);
        let max_lens = Self::max_lengths(lens);

        // create the output lines from title, content and total
        let mut lines: Vec<String> = vec![];
        lines.push(Self::make_head_line(with_size, max_lens));
        for cnt in counters {
            lines.push(cnt.to_string(max_lens));
        }

        // output the total only when there is more than one counters
        if counters.len() > 1 {
            let total_line = Self::make_total_line(total, with_size, max_lens);
            lines.push(total_line);
        }

        // output
        println!("{}", lines.join("\n"));
    }
}

pub fn walk(
    dirpath: &PathBuf,
    with_hidden: bool,
    with_size: bool,
    verbose: bool,
) -> Result<DirDetail> {
    let mut dirs = DirList::new();
    let mut cnt = Counter::new(dirpath, with_size);

    for entry in fs::read_dir(dirpath)? {
        let entry = entry?;
        let path = entry.path();
        let fname = entry.file_name();
        let ftype: String;

        if !with_hidden && fname.to_string_lossy().starts_with('.') {
            // ignore the hidden files and dirs
            continue;
        } else if path.is_symlink() {
            // The size of symbolic link is 0B.
            // So just increase the num of files here.
            cnt.n_files += 1;
            ftype = op::note(&"Symlink");
        } else if path.is_dir() {
            cnt.n_dirs += 1;
            ftype = op::warn(&"Dir");
            dirs.push(path.clone());
        } else {
            cnt.n_files += 1;
            ftype = op::info(&"File");
            // count file size and insert into SizeMap
            if let Some(mp) = cnt.sz_map.as_mut() {
                let meta = entry.metadata()?;
                mp.insert(meta.st_ino(), Counter::file_size(&meta));
            }
        }
        if verbose {
            println!("{:>18} > {}", ftype, path.to_string_lossy());
        }
    }

    return Ok((dirs, cnt));
}

pub fn parallel_walk(
    dirlist: Vec<PathBuf>,
    with_hidden: bool,
    with_size: bool,
    verbose: bool,
    n_thread: usize,
) -> Vec<Counter> {
    let (path_tx, path_rx) = m_channel::<PathBuf>();
    let (cnt_tx, cnt_rx) = s_channel::<Counter>();
    let mut counters = Vec::from_iter(dirlist.iter().map(|p| Counter::new(p, with_size)));
    let stat_locker = Arc::new(Mutex::new(HashMap::new()));

    // send dirlist to path channel
    for path in dirlist {
        path_tx.send(path.clone()).expect("send path err");
    }

    // create walk threads which amount is n_thread
    for t_idx in 0..n_thread {
        // clone channels
        let _path_tx = path_tx.clone();
        let _path_rx = path_rx.clone();
        let _cnt_tx = cnt_tx.clone();
        let _lock = stat_locker.clone();

        // create walk threads
        thread::Builder::new()
            .spawn(move || {
                // get a dir path to traverse
                for dirpath in _path_rx {
                    // switch stat to BUSY
                    {
                        let mut idle_stat = _lock.lock().expect("acquire lock err");
                        idle_stat.insert(t_idx, false);
                    }

                    // traverse all files in the directory
                    let (sub_dirs, sub_cnt) = walk(&dirpath, with_hidden, with_size, verbose)
                        .expect(&format!("walk err: {}", &dirpath.to_str().unwrap()));

                    // send the sub_dirs and the sub_counter back
                    for path in sub_dirs {
                        _path_tx.send(path.clone()).expect("send path err");
                    }
                    _cnt_tx.send(sub_cnt).expect("send counter err");

                    // switch stat to IDLE
                    {
                        let mut idle_stat = _lock.lock().expect("acquire lock err");
                        idle_stat.insert(t_idx, true);
                    }
                }
            })
            .expect("create thread err");
    }

    // check the status
    loop {
        let is_idle: bool;
        {
            let idle_stat = stat_locker.lock().expect("acquire lock err");
            is_idle = idle_stat.values().all(|st| st == &true);
        }

        if is_idle && path_rx.is_empty() {
            break;
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }

    // get the result
    while let Ok(cnt) = cnt_rx.try_recv() {
        for _cnt in &mut counters {
            _cnt.merge(&cnt);
        }
    }

    return counters;
}

#[test]
fn test_readable_size() {
    let mut c = Counter::new(&PathBuf::from("."), true);

    c.sz_map.as_mut().unwrap().insert(1, 1023);
    assert_eq!(c.readable_size(), "1023B");

    c.sz_map.as_mut().unwrap().insert(1, 1434);
    assert_eq!(c.readable_size(), "1.4K");

    c.sz_map.as_mut().unwrap().insert(1, 15926);
    assert_eq!(c.readable_size(), "15.6K");

    c.sz_map.as_mut().unwrap().insert(1, 53589793);
    assert_eq!(c.readable_size(), "51.1M");

    c.sz_map.as_mut().unwrap().insert(1, 238462643383);
    assert_eq!(c.readable_size(), "222.1G");

    c.sz_map.as_mut().unwrap().insert(1, 279502884197169);
    assert_eq!(c.readable_size(), "254.2T");

    c.sz_map.as_mut().unwrap().insert(1, 0xffffffffffffffff);
    assert_eq!(c.readable_size(), "16E");
}
