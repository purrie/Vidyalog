use serde::Deserialize;

use crate::data::Playlist;

pub mod error;

/// Main UI messages
#[derive(Debug, Clone)]
pub enum Message {
    AddPlaylistURL(String),
    AddPlaylist,
    DeletePlaylist(String),
    Test,
    ResultPlaylist(Result<Playlist, Error>),
    OpenInBrowser(String),
}

#[derive(Default, Debug, Clone)]
pub enum WindowScreen {
    #[default]
    PlaylistMod,
}

#[derive(Default, Debug)]
pub enum VideoService {
    #[default]
    Unknown,
}
#[derive(Default, Debug, Clone, Deserialize)]
pub enum Error {
    #[default]
    Unknown,
    ReqwestError(String),
    SerializationError(String),
    DeserializationError(String),
    IOError(String),
    Utf8Error(String),
    InvalidPlaylistURL(String),
    IncompleteResponse,
}
