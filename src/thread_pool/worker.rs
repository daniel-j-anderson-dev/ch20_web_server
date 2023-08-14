use std::thread::{self, JoinHandle};

use crate::{error::Error, thread_pool::Receiver};

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
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
    pub fn new(id: usize, receiver: Receiver) -> Result<Worker, Error> {
        println!("Creating Worker {id}");
        let thread: JoinHandle<()> = thread::Builder::new()
        .spawn(move || loop {
            println!("Worker {id} is waiting for a job message");
            let job_message = match receiver.lock() {
                Ok(guard) => guard,
                Err(error) => {
                    eprintln!("Worker {id} could not lock the receiver: {error}");
                    break;
                },
            }
            .recv();

            match job_message {
                Ok(job) => {
                    println!("Wokrer {id} got a job; executing.");
                    job();
                }
                Err(_error) => {
                    println!("Channel closed; Worker {id} shutting down.");
                    break;
                }
            }
        })?;

        return Ok(Worker {
            id,
            thread: Some(thread),
        });
    }
}
