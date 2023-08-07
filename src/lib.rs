use std::thread;

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

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}
impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread: thread::JoinHandle<()> = thread::spawn(|| {});
        return Worker { id, thread, }
    }
}

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