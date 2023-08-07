pub mod pool_creation_error;
pub mod worker;

use crate::thread_pool::pool_creation_error::PoolCreationError;
use crate::thread_pool::worker::Worker;

type ThreadExecutionError = Box<dyn std::error::Error>;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The pool_size is the number of threads in the returned pool.
    pub fn new(pool_size: usize) -> Result<ThreadPool, PoolCreationError> {
        if pool_size == 0 {
            return Err(PoolCreationError::PoolSizeZero);
        }

        let mut workers: Vec<Worker>  = Vec::with_capacity(pool_size);

        for worker_id in 0..pool_size {
            workers.push(Worker::new(worker_id));
        }
        
        return  Ok(ThreadPool { workers })
    }

    /// Executes the closure on an avliable thread, or it goes in the queue
    /// 
    /// # The closure must return:
    /// A Result
    ///     OK(The unit type)
    ///     Err(A trait object implementing std::error::Error)
    pub fn execute<F>(&self, f: F,) -> Result<(), ThreadExecutionError>
    where
        F: FnOnce() -> Result<(), ThreadExecutionError> + Send + 'static
    {
        return f();
    }
}