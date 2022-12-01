use std::fmt::Display;

use super::Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unknown => write!(f, "Unknown error occured"),
            Error::InvalidPlaylistURL(u) => write!(f, "{} is not a valid playlist URL", u),
            Error::InvalidVideoURL(u) => write!(f, "{} is not a valid video URL", u),
            Error::InvalidThumbnailURL(url) => write!(f, "{} is not a valid thumbnail URL", url),
            Error::IncompleteResponse => write!(f, "Error: Request response was incomplete"),
            Error::MissingID(s) => write!(f, "Missing ID error: {}", s),
            Error::Mismatch(info) => write!(f, "Mismatch error: {info}"),
            Error::ReqwestError(e) => write!(f, "{}", e),
            Error::SerializationError(e) => write!(f, "{}", e),
            Error::IOError(e) => write!(f, "{}", e),
            Error::DeserializationError(e) => write!(f, "{}", e),
            Error::Utf8Error(e) => write!(f, "{}", e),
            Error::ParsingError(e) => write!(f, "{}", e),
            Error::ImageError(e) => write!(f, "{e}"),
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

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParsingError(format!("Parsing error: {}", e))
    }
}
impl From<image::error::ImageError> for Error {
    fn from(e: image::error::ImageError) -> Self {
        Error::ImageError(format!("Image error: {e}"))
    }
}
