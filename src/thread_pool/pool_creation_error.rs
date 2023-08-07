#[derive(Debug)]
pub enum PoolCreationError {
    PoolSizeZero,
}
impl std::fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PoolCreationError::PoolSizeZero => write!(f, "the number of threads in a pool (pool_size) must be at least 1"),
        }
    }
}
impl std::error::Error for PoolCreationError {
    fn description(&self) -> &'static str {
        match self {
            PoolCreationError::PoolSizeZero => return "the number of threads in a pool (pool_size) must be at least 1",
        }
    }
}
