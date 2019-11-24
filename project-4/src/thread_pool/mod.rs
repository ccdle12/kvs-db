use crate::Result;

mod naive;
mod shared_queue;

pub use self::naive::NaiveThreadPool;
pub use self::shared_queue::SharedQueueThreadPool;

/// Trait for Thread Pools.
pub trait ThreadPool {
    /// Creates a new thread pool.
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;

    /// Spawns a function F, into the thread pool.
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
