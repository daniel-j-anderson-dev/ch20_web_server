use std::thread;

use crate::thread_pool::error::Error;

pub struct Worker {
    id: usize,
    thread: std::thread::JoinHandle<()>
}
impl Worker {
    pub fn new(id: usize) -> Result<Worker, Error> {
        let thread: thread::JoinHandle<()> = thread::Builder::new()
            .spawn(|| {})
            .map_err(|io_error| Error::StdIo(io_error))?;
        return Ok(Worker { id, thread, });
    }
}