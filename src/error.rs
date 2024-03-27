use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    ParseUrlError(String),
    FetchError(String),
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseUrlError(e) => write!(f, "Couldn't parse the url: {}", e),
            Error::FetchError(e) => write!(f, "Couldn't fetch data from url: {}", e),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::ParseUrlError(e.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::FetchError(e.to_string())
    }
}
