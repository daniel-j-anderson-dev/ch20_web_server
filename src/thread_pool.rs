use std::sync::{
    mpsc,
    Mutex,
    Arc
};

mod worker;
pub mod job;

use crate::{
    error::Error,
    Error::*,
    thread_pool::worker::Worker,
    thread_pool::job::Job,
};


type Receiver = Arc<Mutex<mpsc::Receiver<Job>>>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The pool_size is the number of threads in the returned pool.
    /// 
    /// pool_size must be greater than 0.
    pub fn new(pool_size: usize) -> Result<ThreadPool, Error> {
        if pool_size == 0 {
            return Err(ThreadPoolSizeZero);
        }

        let (sender, receiver) = mpsc::channel();
        
        // create a counted refrence of a mutual exclus
        let receiver: Receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(pool_size);

        for worker_id in 0..pool_size {
            let receiver_clone: Receiver = Arc::clone(&receiver);
            workers.push(Worker::new(worker_id, receiver_clone)?);
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
    ///     E: Trait object that impl Err(crate::error::Error::MpscSend(std::sync::mpsc::SendError<U>))
    /// 
    /// trait object ex: Box<dyn std::error::Error>
    pub fn execute<F>(&self, task: F,) -> Result<(), Error>
    where
        F: FnOnce() + Send + 'static
    {
        let job: Box<F> = Box::new(task);
        return match self.sender.send(job) {
            Ok(_) => Ok(()),
            Err(value) => Err(MpscSend(value)),
        }
    }
}