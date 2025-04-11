use std::fmt;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseError(String),
    CustomError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "IO Error: {}", err),
            Error::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            Error::CustomError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(err) => Some(err),
            Error::ParseError(_) => None,
            Error::CustomError(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error::CustomError(msg)
    }
}