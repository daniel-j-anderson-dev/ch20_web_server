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

use super::error::Error;

type Job = Box<dyn FnOnce() + Send + 'static>;
type Receiver = Arc<Mutex<mpsc::Receiver<Job>>>;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}
impl Worker {
    pub fn new(id: usize, receiver: Receiver) -> Result<Worker, Error> {
        let thread: JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>> = thread::Builder::new()
            .spawn(|| {
                receiver    
            })
            .map_err(|error| Error::StdIo(error))?;
        return Ok(Worker { id, thread, });
    }
}