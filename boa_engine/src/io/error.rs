use super::StdError;
use core::{fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
pub struct Error {
    kind: ErrorKind,
    error: &'static str,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind.as_str(), self.error)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(deprecated)]
#[non_exhaustive]
pub enum ErrorKind {
    // NotFound,
    // PermissionDenied,
    // ConnectionRefused,
    // ConnectionReset,
    // HostUnreachable,
    // NetworkUnreachable,
    // ConnectionAborted,
    // NotConnected,
    // AddrInUse,
    // AddrNotAvailable,
    // NetworkDown,
    // BrokenPipe,
    // AlreadyExists,
    // WouldBlock,
    // NotADirectory,
    // IsADirectory,
    // DirectoryNotEmpty,
    // ReadOnlyFilesystem,
    // FilesystemLoop,
    // StaleNetworkFileHandle,
    // InvalidInput,
    InvalidData,
    // TimedOut,
    // WriteZero,
    // StorageFull,
    // NotSeekable,
    // FilesystemQuotaExceeded,
    // FileTooLarge,
    // ResourceBusy,
    // ExecutableFileBusy,
    // Deadlock,
    // CrossesDevices,
    // TooManyLinks,
    // InvalidFilename,
    // ArgumentListTooLong,
    // Interrupted,
    // Unsupported,
    UnexpectedEof,
    // OutOfMemory,
    // Other,
    // Uncategorized,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl Error {
    pub fn new(kind: ErrorKind, error: &'static str) -> Error {
        Error { kind, error }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        use ErrorKind::*;
        match *self {
            // AddrInUse => "address in use",
            // AddrNotAvailable => "address not available",
            // AlreadyExists => "entity already exists",
            // ArgumentListTooLong => "argument list too long",
            // BrokenPipe => "broken pipe",
            // ConnectionAborted => "connection aborted",
            // ConnectionRefused => "connection refused",
            // ConnectionReset => "connection reset",
            // CrossesDevices => "cross-device link or rename",
            // Deadlock => "deadlock",
            // DirectoryNotEmpty => "directory not empty",
            // ExecutableFileBusy => "executable file busy",
            // FileTooLarge => "file too large",
            // FilesystemLoop => "filesystem loop or indirection limit (e.g. symlink loop)",
            // FilesystemQuotaExceeded => "filesystem quota exceeded",
            // HostUnreachable => "host unreachable",
            // Interrupted => "operation interrupted",
            InvalidData => "invalid data",
            // InvalidFilename => "invalid filename",
            // InvalidInput => "invalid input parameter",
            // IsADirectory => "is a directory",
            // NetworkDown => "network down",
            // NetworkUnreachable => "network unreachable",
            // NotADirectory => "not a directory",
            // NotConnected => "not connected",
            // NotFound => "entity not found",
            // NotSeekable => "seek on unseekable file",
            // Other => "other error",
            // OutOfMemory => "out of memory",
            // PermissionDenied => "permission denied",
            // ReadOnlyFilesystem => "read-only filesystem or storage medium",
            // ResourceBusy => "resource busy",
            // StaleNetworkFileHandle => "stale network file handle",
            // StorageFull => "no storage space",
            // TimedOut => "timed out",
            // TooManyLinks => "too many links",
            // Uncategorized => "uncategorized error",
            UnexpectedEof => "unexpected end of file",
            // Unsupported => "unsupported",
            // WouldBlock => "operation would block",
            // WriteZero => "write zero",
        }
    }
}
