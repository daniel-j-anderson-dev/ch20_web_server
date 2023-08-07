use std::{
    thread::{
        self,
        JoinHandle
    },
    sync::{
        mpsc,
        Arc,
        Mutex
    },
};

use crate::error::Error;

type Job = Box<dyn FnOnce() + Send + 'static>;
type Receiver = Arc<Mutex<mpsc::Receiver<Job>>>;

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
                    let job = receiver.lock().unwrap().recv().unwrap();
                    
                    println!("Worker {id} got a job; executing\n");

                    job();
                }
            })
            .map_err(|error| Error::Io(error))?;
        return Ok(Worker { id, thread, });
    }
}