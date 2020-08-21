use crate::thread_pool::ThreadPool;
use crate::KvsError;
pub use crate::Result;

pub struct RayonThreadPool(rayon::ThreadPool);

impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> Result<Self> {
        Ok(RayonThreadPool(
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads as usize)
                .build()
                .map_err(|e| KvsError::StringErr(format!("{}", e)))?,
        ))
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.0.spawn(job);
    }
}
