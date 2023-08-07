use std::thread::{self, Thread};

#[derive(Debug)]
pub enum PoolCreationError {
    PoolSizeZero,
}
impl std::fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PoolCreationError::PoolSizeZero => write!(f, "the number of threads in a pool (pool_size) must be at least 1"),
        }
    }
}
impl std::error::Error for PoolCreationError {
    fn description(&self) -> &'static str {
        match self {
            PoolCreationError::PoolSizeZero => return "the number of threads in a pool (pool_size) must be at least 1",
        }
    }
}

type ThreadExecutionError = Box<dyn std::error::Error>;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The pool_size is the number of threads in the returned pool.
    pub fn new(pool_size: usize) -> Result<ThreadPool, PoolCreationError> {
        if pool_size == 0 {
            return Err(PoolCreationError::PoolSizeZero);
        }

        let workers: Vec<thread::JoinHandle<()>>  = Vec::with_capacity(pool_size);
        
        return  Ok(ThreadPool { workers, })
    }
    pub fn execute<F>(&self, f: F,) -> Result<(String, String), ThreadExecutionError>
    where
        F: FnOnce() -> Result<(String, String), ThreadExecutionError> + Send + 'static
    {
        return f();
    }
}