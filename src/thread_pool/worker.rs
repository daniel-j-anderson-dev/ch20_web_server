use std::{
    thread::{
        self,
        JoinHandle
    },
    sync::{
        mpsc,
        MutexGuard
    },
};

use crate::{
    error::Error,
    Error::*,
    thread_pool::Receiver,
    thread_pool::Job,
};

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}
impl Worker {
    /// Executes the closure on an avliable thread, or it goes in the queue
    /// 
    /// The closure must return a Result<T, E>
    /// 
    /// where
    /// 
    ///     T: ()
    /// 
    ///     E: crate::thread_pool:error::Error
    /// 
    /// trait object ex: Box<dyn std::error::Error>
    pub fn new(id: usize, receiver: Receiver) -> Result<Worker, Error> {
        let thread: JoinHandle<()> = thread::Builder::new()
            .spawn(move || {
                loop {
                    // let job = receiver.lock().unwrap().recv().unwrap(); 
                    let job_lock: MutexGuard<'_, mpsc::Receiver<Job>> = match receiver.lock() {
                        Ok(lock) => lock,
                        Err(error) => {
                            eprintln!("Worker {} couldn't get a lock on the job reciever: {}", id, Poision(error.to_string()));
                            continue;
                        },
                    };

                    let job: Job = match job_lock.recv() {
                        Ok(job) => job, 
                        Err(error) => {
                            eprintln!("Worker {} couldn't get a job: {}", id, Recv(error));
                            continue;
                        }
                    };
                    
                    println!("Worker {id} got a job; executing\n");

                    job();
                }
            })
            .map_err(|error| Error::Io(error))?;
        return Ok(Worker { id, thread, });
    }
}