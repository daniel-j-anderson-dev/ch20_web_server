#[derive(Debug)]
pub enum Error {
    PoolSizeZero,
}
impl std::fmt::Display for crate::thread_pool::Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PoolSizeZero => write!(f, "the number of threads in a pool (pool_size) must be at least 1"),
        }
    }
}
impl std::error::Error for crate::thread_pool::Error {
    fn description(&self) -> &'static str {
        match self {
            Error::PoolSizeZero => return "the number of threads in a pool (pool_size) must be at least 1",
        }
    }
}
