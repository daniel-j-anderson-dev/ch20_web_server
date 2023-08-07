use std::sync::{
    mpsc,
    Mutex,
    Arc
};

pub mod error;
mod worker;
mod job;

use self::error::Error;
use self::worker::Worker;
use self::job::Job;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The pool_size is the number of threads in the returned pool.
    pub fn new(pool_size: usize) -> Result<ThreadPool, Error> {
        if pool_size == 0 {
            return Err(Error::PoolSizeZero);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker>  = Vec::with_capacity(pool_size);

        for worker_id in 0..pool_size {
            workers.push(Worker::new(worker_id, Arc::clone(&receiver))?);
        }
        
        return  Ok(ThreadPool { workers, sender })
    }

    /// Executes the closure on an avliable thread, or it goes in the queue
    /// 
    /// The closure must return a Result<T, E>
    /// 
    /// where
    /// 
    ///     T: ()
    /// 
    ///     E: Trait object that impl std::error::Error
    /// 
    /// trait object ex: Box<dyn std::error::Error>
    pub fn execute<F>(&self, f: F,) -> Result<(), Error>
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        return match self.sender.send(job) {
            Ok(_) => Ok(()),
            Err(error) => Err(Error::Send(error)),
        }
    }
}