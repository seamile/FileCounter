use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::RecvError;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[allow(unused)]
struct Worker {
    inbox: (Sender<PathBuf>, Receiver<PathBuf>),
    outbox: Sender<u64>,
    handler: JoinHandle<()>,
}

#[allow(unused)]
impl Worker {
    pub fn new(result_tx: Sender<u64>) -> Worker {
        return Worker {
            inbox: channel::<PathBuf>(),
            outbox: result_tx,
            handler: thread::spawn(|| {}),
        };
    }

    pub fn put_task(&self, dirpath: PathBuf) {
        self.inbox.0.send(dirpath);
    }

    fn get_task(&self) -> Result<PathBuf, RecvError> {
        return self.inbox.1.recv();
    }

    fn put_result(&self, result: u64) {
        self.outbox.send(result);
    }

    // 遍历目录
    fn walk(&self) {}

    pub fn run(&self) {
        loop {
            if let Ok(item) = self.get_task() {
                todo!()
            };
        }
    }
}

pub fn schedule(n_thread: u8) {
    let (result_sender, result_receiver) = channel::<u64>();

    // 创建线程池
    let mut pool: Vec<Worker> = vec![];
    for _ in 0..n_thread {
        pool.push(Worker::new(result_sender.clone()));
    }
    let inv = Duration::from_millis(1);
    let res = result_receiver.recv_timeout(inv);
}
