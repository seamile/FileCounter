use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::sync::mpsc::channel as s_channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use flume::unbounded as m_channel;

#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;
#[cfg(target_os = "macos")]
use std::os::macos::fs::MetadataExt;
#[cfg(target_os = "unix")]
use std::os::unix::fs::MetadataExt;

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

    fn name(&self) -> &str {
        return self.dirpath.to_str().expect("dir path err");
    }

    fn file_size(metadata: &fs::Metadata) -> u64 {
        let sz = metadata.st_size() as f64;
        let blksz = metadata.st_blksize() as f64;
        return (blksz * (sz / blksz).ceil()) as u64;
    }

    pub fn size(&self) -> u64 {
        match self.sz_map.as_ref() {
            Some(mp) => mp.values().sum(),
            None => 0,
        }
    }

    // Make "size" more readable
    fn readable_size(&self) -> String {
        let mut sz = self.size() as f64;
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

    fn lengths(&self) -> Lengths {
        return (
            op::display_width(&self.name().to_string()),
            self.n_files.to_string().len(),
            self.n_dirs.to_string().len(),
            self.readable_size().len(),
        );
    }

    fn make_title(with_size: bool, lens: Lengths) -> String {
        let f0 = op::align_left(&"Name", lens.0);
        let f1 = op::align_right(&"Files", lens.1);
        let f2 = op::align_right(&"Dirs", lens.2);
        if with_size {
            let f3 = op::align_right(&"Size", lens.3);
            return op::title(&vec![f0, f1, f2, f3].join("  "));
        } else {
            return op::title(&vec![f0, f1, f2].join("  "));
        }
    }

    fn join_fields(&self, lens: Lengths) -> String {
        let f0 = op::align_left(&self.name(), lens.0);
        let f1 = op::align_right(&self.n_files, lens.1);
        let f2 = op::align_right(&self.n_dirs, lens.2);
        if self.sz_map == None {
            return vec![f0, f1, f2].join("  ");
        } else {
            let f3 = op::align_right(&self.readable_size(), lens.3);
            return vec![f0, f1, f2, f3].join("  ");
        }
    }

    pub fn output(counters: &Vec<Self>, with_size: bool) {
        let mut lines: Vec<String> = vec![];
        let lens = counters
            .iter()
            .map(|c| c.lengths())
            .map(|w| (w.0.max(4), w.1.max(5), w.2.max(4), w.3.max(4)))
            .reduce(|m, n| (n.0.max(m.0), n.1.max(m.1), n.2.max(m.2), n.3.max(m.3)))
            .unwrap();

        // make title and content lines
        lines.push(op::title(&Self::make_title(with_size, lens)));
        for cnt in counters {
            lines.push(cnt.join_fields(lens));
        }

        // output
        println!("{}", lines.join("\n"));
    }
}

pub fn walk(dirpath: &PathBuf, with_hidden: bool, with_size: bool) -> Result<DirDetail> {
    let mut dirs = DirList::new();
    let mut cnt = Counter::new(dirpath, with_size);

    for entry in fs::read_dir(dirpath)? {
        let entry = entry?;
        let path = entry.path();
        let fname = entry.file_name();

        if !with_hidden && fname.to_string_lossy().starts_with('.') {
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
            // count file size and insert into SizeMap
            if let Some(mp) = cnt.sz_map.as_mut() {
                let meta = entry.metadata()?;
                mp.insert(meta.st_ino(), Counter::file_size(&meta));
            }
        }
    }

    return Ok((dirs, cnt));
}

type Locker = Arc<Mutex<HashMap<usize, bool>>>;

pub fn parallel_walk(
    dirlist: Vec<PathBuf>,
    with_hidden: bool,
    with_size: bool,
    n_thread: usize,
) -> Vec<Counter> {
    let (path_tx, path_rx) = m_channel::<PathBuf>();
    let (cnt_tx, cnt_rx) = s_channel::<Counter>();
    let mut counters = Vec::from_iter(dirlist.iter().map(|p| Counter::new(p, with_size)));
    let stat_locker: Locker = Arc::new(Mutex::new(HashMap::new()));

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
                    let (sub_dirs, sub_cnt) = walk(&dirpath, with_hidden, with_size)
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
