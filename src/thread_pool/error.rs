pub type StdError = Box<dyn std::error::Error>;
pub type SendError = std::sync::mpsc::SendError<Box<(dyn FnOnce() + Send + 'static)>>;

#[derive(Debug)]
pub enum Error {
    PoolSizeZero,
    StdIo(std::io::Error),
    Send(SendError)
}
impl Error {
    pub fn to_str(&self) -> &'static str {
        match self {
            Error::PoolSizeZero => "Number of threads (pool_number) must be at least 1",
            Error::StdIo(error) => std_io_error_to_str(error),
            Error::Send(error) => {
                println!("{}", error.to_string());
                return "std::sync::mpsc::SendError";
            },
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
impl std::error::Error for Error {
    fn description(&self) -> &'static str {
        self.to_str()
    }
}

fn std_io_error_to_str(error: &std::io::Error) -> &'static str {
    return match error.kind() {
        std::io::ErrorKind::NotFound => "std::io::Error: NotFound",
        std::io::ErrorKind::PermissionDenied => "std::io::Error: PermissionDenied",
        std::io::ErrorKind::ConnectionRefused => "std::io::Error: ConnectionRefused",
        std::io::ErrorKind::ConnectionReset => "std::io::Error: ConnectionReset",
        std::io::ErrorKind::ConnectionAborted => "std::io::Error: ConnectionAborted",
        std::io::ErrorKind::NotConnected => "std::io::Error: NotConnected",
        std::io::ErrorKind::AddrInUse => "std::io::Error: AddrInUse",
        std::io::ErrorKind::AddrNotAvailable => "std::io::Error: AddrNotAvailable",
        std::io::ErrorKind::BrokenPipe => "std::io::Error: BrokenPipe",
        std::io::ErrorKind::AlreadyExists => "std::io::Error: AlreadyExists",
        std::io::ErrorKind::WouldBlock => "std::io::Error: WouldBlock",
        std::io::ErrorKind::InvalidInput => "std::io::Error: InvalidInput",
        std::io::ErrorKind::InvalidData => "std::io::Error: InvalidData",
        std::io::ErrorKind::TimedOut => "std::io::Error: TimedOut",
        std::io::ErrorKind::WriteZero => "std::io::Error: WriteZero",
        std::io::ErrorKind::Interrupted => "std::io::Error: Interrupted",
        std::io::ErrorKind::Unsupported => "std::io::Error: Unsupported",
        std::io::ErrorKind::UnexpectedEof => "std::io::Error: UnexpectedEof",
        std::io::ErrorKind::OutOfMemory => "std::io::Error: OutOfMemory",
        std::io::ErrorKind::Other => "std::io::Error: Other",
        //unstable io ErrorKinds
        // std::io::ErrorKind::HostUnreachable => "std::io::Error: HostUnreachable",
        // std::io::ErrorKind::NetworkUnreachable => "std::io::Error: NetworkUnreachable",
        // std::io::ErrorKind::NetworkDown => "std::io::Error: NetworkDown",
        // std::io::ErrorKind::NotADirectory => "std::io::Error: NotADirectory",
        // std::io::ErrorKind::IsADirectory => "std::io::Error: IsADirectory",
        // std::io::ErrorKind::DirectoryNotEmpty => "std::io::Error: DirectoryNotEmpty",
        // std::io::ErrorKind::ReadOnlyFilesystem => "std::io::Error: ReadOnlyFilesystem",
        // std::io::ErrorKind::FilesystemLoop => "std::io::Error: FilesystemLoop",
        // std::io::ErrorKind::StaleNetworkFileHandle => "std::io::Error: StaleNetworkFileHandle",
        // std::io::ErrorKind::StorageFull => "std::io::Error: StorageFull",
        // std::io::ErrorKind::NotSeekable => "std::io::Error: NotSeekable",
        // std::io::ErrorKind::FilesystemQuotaExceeded => "std::io::Error: FilesystemQuotaExceeded",
        // std::io::ErrorKind::FileTooLarge => "std::io::Error: FileTooLarge",
        // std::io::ErrorKind::ResourceBusy => "std::io::Error: ResourceBusy",
        // std::io::ErrorKind::ExecutableFileBusy => "std::io::Error: ExecutableFileBusy",
        // std::io::ErrorKind::Deadlock => "std::io::Error: Deadlock",
        // std::io::ErrorKind::CrossesDevices => "std::io::Error: CrossesDevices",
        // std::io::ErrorKind::TooManyLinks => "std::io::Error: TooManyLinks",
        // std::io::ErrorKind::InvalidFilename => "std::io::Error: InvalidFilename",
        // std::io::ErrorKind::ArgumentListTooLong => "std::io::Error: ArgumentListTooLong",
        // std::io::ErrorKind::Uncategorized => "std::io::Error: Uncategorized",
        _ => "std io error"
    }
}
