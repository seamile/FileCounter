use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::RecvError;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::walker as wk;

#[allow(unused)]
struct Worker {
    inbox: (Sender<PathBuf>, Receiver<PathBuf>),
    outbox: Sender<wk::DirDetail>,
    idle: bool,
    handler: Option<JoinHandle<()>>,
}

#[allow(unused)]
impl Worker {
    pub fn new(result_tx: Sender<wk::DirDetail>) -> Worker {
        return Worker {
            inbox: channel::<PathBuf>(),
            outbox: result_tx,
            idle: true,
            handler: None,
        };
    }

    pub fn is_idle(&self) -> bool {
        self.idle
    }

    pub fn put_task(&self, dirpath: PathBuf) {
        self.inbox.0.send(dirpath);
    }

    fn get_task(&self) -> Result<PathBuf, RecvError> {
        return self.inbox.1.recv();
    }

    fn put_result(&self, result: wk::DirDetail) {
        self.outbox.send(result);
    }

    pub fn run(&mut self, ignore_hidden: bool, count_sz: bool) {
        loop {
            if let Ok(dirpath) = self.get_task() {
                self.idle = false;
                if let Ok(res) = wk::walk(&dirpath, ignore_hidden, count_sz) {
                    self.put_result(res);
                };
            };
            self.idle = true;
        }
    }

    // pub fn foo(&self) {
    //     let _self = Rc::new(self);
    //     let t = thread::spawn(move || _self.run(true, true));
    // }

    pub fn launch(mut wk: Worker, ignore_hidden: bool, count_sz: bool) -> JoinHandle<()> {
        thread::spawn(move || wk.run(ignore_hidden, count_sz))
    }
}

struct Foo {
    a: i32,
}

impl Foo {
    pub fn bar(&mut self) {
        self.a += 1;
        println!("Current a = {}", self.a);
    }

    pub fn test<'a>(&'a mut self) {
        thread::spawn(self.bar());
    }
}

pub fn schedule(dirpath: PathBuf, ignore_hidden: bool, count_sz: bool, n_thread: u8) {
    let mut counter = wk::Counter::new(&dirpath);
    let (result_tx, result_rx) = channel::<wk::DirDetail>();

    // create the thread pool
    let mut pool: Vec<&Worker> = vec![];
    for _ in 0..n_thread {
        let worker = Worker::new(result_tx.clone());
        // pool.push(&worker);
        Worker::launch(worker, ignore_hidden, count_sz);
    }

    if let Ok((dirs, cnt)) = result_rx.recv_timeout(Duration::from_millis(1)) {
        counter.update(cnt);
        if !dirs.is_empty() {
            let mut idir = dirs.iter();
            for worker in pool {
                if worker.is_idle() {
                    if let Some(path) = idir.next() {
                        worker.put_task(path.to_path_buf());
                    }
                }
            }
        }
    };
}
