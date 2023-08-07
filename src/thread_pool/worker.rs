pub struct Worker {
    id: usize,
    thread: std::thread::JoinHandle<()>
}
impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread = std::thread::spawn(|| {});
        return Worker { id, thread, }
    }
}