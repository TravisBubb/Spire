//! # Errors

/// Spire error type
#[derive(Debug)]
pub enum Error {
    /// Wrapper around `std::io::Error`
    Io(std::io::Error),
    TooManyArguments(usize),
    UnrecognizedOption(String),
}

impl From<std::io::Error> for Error {
    /// Convert a `std::io::Error` into a Spire Error
    fn from(err: std::io::Error) -> Self { Self::Io(err) }
}
