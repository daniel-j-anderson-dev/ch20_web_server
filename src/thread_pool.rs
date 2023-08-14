use std::sync::{mpsc, Arc, Mutex};

pub mod job;
mod worker;

use crate::{error::Error, thread_pool::job::Job, thread_pool::worker::Worker, Error::*};

type Receiver = Arc<Mutex<mpsc::Receiver<Job>>>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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

        return Ok(ThreadPool {
            workers,
            sender: Some(sender),
        });
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
    pub fn execute<F>(&self, job: F) -> Result<(), Error>
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Box<F> = Box::new(job);

        return match &self.sender {
            Some(sender) => sender.send(job).map_err(|error| MpscSend(error)),
            None => Err(std::io::Error::new(std::io::ErrorKind::Other, "No sender. was it dropped?").into())
        };
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Thread pool is closing the channel");
        drop(self.sender.take());

        for worker in self.workers.iter_mut() {
            if let Some(thread) = worker.thread.take() {
                match thread.join() {
                    Ok(_) => println!("Thread pool joining worker {}'s thread", worker.id),
                    Err(error) => eprintln!("Error Shutting down worker: {:?}", error)
                }
            }
        }
        println!("All worker threads joined main thread. Dropping the thread pool");
    }
}