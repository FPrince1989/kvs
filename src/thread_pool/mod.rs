pub use crate::Result;

mod naive;

pub use self::naive::NaiveThreadPool;

/// defines the thread pool interface
pub trait ThreadPool {
    /// create a thread pool
    fn new(threads: u32) -> Result<Self>;

    ///spawn a function in the thread pool
    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static;
}