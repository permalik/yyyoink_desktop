use std::{fmt, io::ErrorKind};

#[derive(Debug, Clone)]
pub enum Error {
    PermissionDenied,
    FileNotFound,
    IoError(ErrorKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PermissionDenied => write!(f, "Permission Denied"),
            Error::FileNotFound => write!(f, "File Not Found"),
            Error::IoError(kind) => write!(f, "IO Error: {:?}", kind),
        }
    }
}
