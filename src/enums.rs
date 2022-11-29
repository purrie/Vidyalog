use serde::{Deserialize, Serialize};

use crate::{
    data::{Playlist, Video},
    service::ContentIdentifier,
};

pub mod error;
mod video_service;
mod video_status;

/// Main UI messages
#[derive(Debug, Clone)]
pub enum Message {
    AddPlaylistURL(String),
    QueryPlaylist,
    DeletePlaylist(ContentIdentifier<Playlist>),
    AddPlaylist(Result<Playlist, Error>),
    AddVideo(Result<Video, Error>),
    OpenInBrowser(String),
    OpenVideoExternally(ContentIdentifier<Video>),
    ToggleWatchStatus(ContentIdentifier<Video>),
    OpenScreen(WindowScreen),
    SetTheme(iced::Theme),
    ToggleTracking(ContentIdentifier<Playlist>),
}

#[derive(Default, Debug, Clone)]
pub enum WindowScreen {
    #[default]
    Home,
    PlaylistTracker,
    PlaylistDetail(ContentIdentifier<Playlist>),
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
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
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum VideoStatus {
    /// Marks video as not seen, new basically.
    #[default]
    Unseen,
    /// Marks the video as having been opened externally but there's no additional watch information
    Browsed,
    /// Marks video as seen to completion
    Watched,
}
