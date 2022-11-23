use std::fmt::Display;

use super::Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unknown => write!(f, "Unknown error occured"),
            Error::InvalidPlaylistURL(u) => write!(f, "{} is not a valid playlist URL", u),
            Error::IncompleteResponse => write!(f, "Error: Request response was incomplete"),
            Error::ReqwestError(e) => write!(f, "{}", e),
            Error::SerializationError(e) => write!(f, "{}", e),
            Error::IOError(e) => write!(f, "{}", e),
            Error::DeserializationError(e) => write!(f, "{}", e),
            Error::Utf8Error(e) => write!(f, "{}", e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error::ReqwestError(format!("Network Error: {}", error.to_string()))
    }
}

impl From<ron::Error> for Error {
    fn from(error: ron::Error) -> Self {
        Error::SerializationError(format!("Serialization error: {}", error))
    }
}

impl From<ron::error::SpannedError> for Error {
    fn from(e: ron::error::SpannedError) -> Self {
        Error::DeserializationError(format!("Deserialization error: {}", e))
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(format!("I/O Error: {}", error))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Error::Utf8Error(format!("UTF8 Error: {}", e))
    }
}
