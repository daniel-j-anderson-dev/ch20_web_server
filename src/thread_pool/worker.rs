use std::thread::{
    self,
    JoinHandle
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
                    let job: Job = receiver
                        
                        .lock()
                            .unwrap_or_else(|error| {
                                println!("{error}");
                                std::process::exit(2);
                            })

                        .recv()
                            .unwrap_or_else(|error| {
                                println!("{error}");
                                std::process::exit(2);
                            }); 
                    
                    println!("Worker {id} got a job; executing\n");

                    job();
                }
            })
            .map_err(|error| Error::Io(error))?;
        return Ok(Worker { id, thread, });
    }
}