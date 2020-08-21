pub use crate::Result;

mod naive;
mod rayon;
mod shared_queue;

pub use self::naive::NaiveThreadPool;
pub use self::rayon::RayonThreadPool;
pub use self::shared_queue::SharedQueueThreadPool;

/// defines the thread pool interface
pub trait ThreadPool {
    /// create a thread pool
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;

    ///spawn a function in the thread pool
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
