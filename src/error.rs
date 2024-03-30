//! # Error module
//!
//! Contains error types for the library

use std::fmt::Debug;

/// A type alias for `Result<T, Error>`
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the library
pub enum Error {
    /// Represents an error that occurred while processing a response
    ResponseError(String),

    /// Represents an error that occurred while parsing a url
    ParseUrlError(String),

    /// Represents an error that occurred while fetching data from a url
    FetchError(String),

    /// Represents an error that occurred when no parameters were provided for a query
    NoParams(String),

    /// Represents an error that occurred when no results were found for a query
    NoResults(String),
}

/// For debugging purposes
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ResponseError(e) => write!(f, "Response error: {}", e),
            Error::ParseUrlError(e) => write!(f, "Couldn't parse the url: {}", e),
            Error::FetchError(e) => write!(f, "Couldn't fetch data from url: {}", e),
            Error::NoParams(e) => write!(f, "No parameters provided for {} query", e),
            Error::NoResults(e) => write!(f, "No results found for query {}", e),
        }
    }
}

/// Implemention of `url::ParseError`  into() for `Error`
impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::ParseUrlError(e.to_string())
    }
}

/// Implemention of `reqwest::Error` into() for `Error`
#[cfg(any(feature = "pkgs", feature = "pkgs-async"))]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::FetchError(e.to_string())
    }
}
