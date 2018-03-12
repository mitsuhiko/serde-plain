use std::fmt;
use serde::ser;

use std::error;

/// Errors created from this crate.
#[derive(Debug, Clone)]
pub enum Error {
    /// An impossible / unsupported operation was attempted.
    ImpossibleSerialization,
    /// An arbitrary error message.
    Message(String),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Message(msg.to_string())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ImpossibleSerialization => "value cannot be serialized to a plain value",
            Error::Message(ref msg) => msg.as_str(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "plain serialization error: {}", self.description())
    }
}
