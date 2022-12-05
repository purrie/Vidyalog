use serde::{Deserialize, Serialize};

use crate::{
    data::{Playlist, Thumbnail, Video},
    service::ContentIdentifier,
};

pub mod error;
mod video_service;
mod video_status;
mod tooltips;
mod colortheme;

/// Main application events
#[derive(Debug, Clone)]
pub enum Message {
    /// Event synchronises the URL string user is typing or pasting to add a new playlist to the program
    AddPlaylistURL(String),
    /// This event starts the procedure of pulling the playlist from inserted URL and downloading the information about the playlist
    QueryPlaylist,
    /// Event to delete a playlist by its ID
    DeletePlaylist(ContentIdentifier<Playlist>),
    /// Result of a playlist query, adds the resulting playlist to the database on success
    AddPlaylist(Result<Playlist, Error>),
    /// Result of downloading information about a video
    AddVideo(Result<Video, Error>),
    /// Result of downloading information about thumbnail
    AddThumbnail(Result<Thumbnail, Error>),
    /// Event for opening a specific URL in a default system browser. The string is expected to be a valid URL.
    OpenInBrowser(String),
    /// Event to open a video in a default system browser, it also ensures the video information is updated accordingly.
    OpenVideoExternally(ContentIdentifier<Video>),
    /// Event used to change watched status fo a video. It changes the status between watched and unwatched.
    ToggleWatchStatus(ContentIdentifier<Video>),
    /// Event to open specific program UI screen.
    OpenScreen(WindowScreen),
    /// Sets a theme for the program.
    SetTheme(ColorTheme),
    /// Event to toggle whatever the playlist should be tracked on the home screen or not.
    ToggleTracking(ContentIdentifier<Playlist>),
}

/// UI Screen identifiers
///
/// Each value represents different screen in the UI
#[derive(Default, Debug, Clone)]
pub enum WindowScreen {
    /// Home screen is the default first screen to be displayed, its purpose is to agregate unwatched videos for quick and easy access
    #[default]
    Home,
    /// Screen that lists all the playlists along with controls for adding new ones.
    PlaylistTracker,
    /// Screen for displaying details about specific playlist, together with a list of its videos.
    PlaylistDetail(ContentIdentifier<Playlist>),
    /// Screen with all of the options for customizing look and behaviour of the program.
    Settings,
}

/// Markers for supported services from which videos, playlists and other data are pulled.
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum VideoService {
    /// Unknown service, usually it will default to being treated as Youtube.
    #[default]
    Unknown,
    /// Youtube.com
    Youtube,
}
/// Type of the data used for distinguishing information in some places.
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum ContentType {
    /// Represents invalid data, usually means serialization or deserialization issues.
    #[default]
    Invalid,
    /// Represents a playlist, a collection of videos stringed in a sequence.
    Playlist,
    /// A singular video
    Video,
    /// A thumbnail or other image for visually representing other content types.
    Thumbnail,
}
/// Errors that may occur during the execution of program along with the data elaborating on the error.
#[derive(Default, Debug, Clone, Deserialize)]
pub enum Error {
    /// No information about error. This should not be used outside of quick debugging tasks or as a placeholder.
    #[default]
    Unknown,
    /// A Network error originating from the Request framework
    ReqwestError(String),
    /// Error coming from serialization of the data.
    SerializationError(String),
    /// Error coming from deserialization of the data.
    DeserializationError(String),
    /// Error that occurs while writing or reading from the file system.
    IOError(String),
    /// Error that occurs while parsing strings.
    Utf8Error(String),
    /// Error that occurs while the provided URL does not correspond to a video from a known service. Or the URL is malformed.
    InvalidVideoURL(String),
    /// Error that occurs while the provided URL does not correspond to a playlist from a known service. Or the URL is malformed.
    InvalidPlaylistURL(String),
    /// Error that occurs while the provided URL does not correspond to a thumbnail from a known service. Or the URL is malformed.
    InvalidThumbnailURL(String),
    /// Received response to an URL request did not return an expected result.
    IncompleteResponse,
    /// Error while parsing values into string, usually numbers.
    ParsingError(String),
    /// Error while processing images using Image framework.
    ImageError(String),
    /// Error that occurs where the value provided was different than expected.
    Mismatch(String),
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

/// Values used to easily provide text to a tooltip UI widgets, they are created to be implicitly converted into text by the widget.
pub enum TooltipText {
    OpenInBrowser,
    ChangeStatus,
    PlaylistDetails,
    ContinueWatching,
    TrackPlaylist,
    UntrackPlaylist,
    Delete
}

/// Value representing the color theme of the program.
#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum ColorTheme {
    /// Light theme, directly maps to iced::Theme::Light
    #[default]
    Light,
    /// Dark theme, directly maps to iced::Theme::Dark
    Dark,
}
