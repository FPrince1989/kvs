use std::thread;

use crossbeam::channel::{unbounded, Receiver, Sender};
use slog::debug;

use crate::thread_pool::ThreadPool;
pub use crate::Result;
use crate::LOGGING;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct SharedQueueThreadPool {
    sender: Sender<Job>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<Self> {
        let (s, r) = unbounded::<Job>();

        for _ in 0..threads {
            let job_receiver = JobReceiver(r.clone());
            thread::Builder::new().spawn(move || run_job(job_receiver))?;
        }

        Ok(SharedQueueThreadPool { sender: s })
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .send(Box::new(job))
            .expect("Thread pool is full.");
    }
}

#[derive(Clone)]
struct JobReceiver(Receiver<Job>);

impl Drop for JobReceiver {
    fn drop(&mut self) {
        if thread::panicking() {
            let r = self.clone();
            thread::Builder::new()
                .spawn(move || run_job(r))
                .expect("Restart Fail.");
        }
    }
}

fn run_job(r: JobReceiver) {
    loop {
        match r.0.recv() {
            Ok(job) => job(),
            Err(_) => debug!(LOGGING.logger, "Receive Error."),
        }
    }
}
