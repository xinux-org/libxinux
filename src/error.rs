use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    ResponseError(String),
    ParseUrlError(String),
    FetchError(String),
    NoParams(String),
    NoResults(String),
}

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

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::ParseUrlError(e.to_string())
    }
}

#[cfg(any(feature = "pkgs", feature = "pkgs-async"))]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::FetchError(e.to_string())
    }
}
