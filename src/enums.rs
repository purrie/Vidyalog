use serde::{Deserialize, Serialize};

use crate::data::{Playlist, Video};

pub mod error;
mod video_status;

/// Main UI messages
#[derive(Debug, Clone)]
pub enum Message {
    AddPlaylistURL(String),
    AddPlaylist,
    DeletePlaylist(String),
    ResultPlaylist(Result<Playlist, Error>),
    ResultVideo(Result<Video, Error>),
    OpenInBrowser(String),
    OpenScreen(WindowScreen),
    UpdatePlaylist(Result<Playlist, Error>),
    UpdateVideo(Result<Video, Error>),
}

#[derive(Default, Debug, Clone)]
pub enum WindowScreen {
    #[default]
    PlaylistTracker,
    PlaylistDetail(String),
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub enum VideoService {
    #[default]
    Unknown,
    Youtube,
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
    InvalidVideoURL(String),
    InvalidPlaylistURL(String),
    IncompleteResponse,
    ParsingError(String),
    MissingID(String),
}

/// Flag for videos to help distinguish watched from new videos.
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub enum VideoStatus {
    /// Marks video as not seen, new basically.
    #[default]
    Unseen,
    /// This marks the video as previously opened but not necessarily seen to completion.
    Seen(u32),
    /// Marks video as seen to completion
    Watched,
}
