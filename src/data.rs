use iced::widget::image;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{VideoService, VideoStatus},
    service::ContentIdentifier,
};

mod playlist;
mod thumbnail;
mod video;

/// Contains all the information necessary for tracking and playing a playlist
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub struct Playlist {
    /// URL corresponding to the playlist
    pub url: String,
    /// Identification number as used by the service from which the playlist originates
    pub id: String,
    /// Title of the playlist, holds the user presentable name
    pub title: String,
    /// Holds the description of the playlist
    pub description: String,
    /// Name of the author of the playlist
    pub author: String,
    /// Contains ID numbers for the videos
    pub videos: Vec<ContentIdentifier<Video>>,
    /// Marks from which service the playlist originates
    pub source: VideoService,
    /// Flag declaring whatever the playlist should be tracked on the home page or not
    pub tracked: bool,
    /// Identificator for the playlist thumbnail
    pub thumbnail: ContentIdentifier<Thumbnail>,
}

/// Contains all the data relevant to a single video
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub struct Video {
    /// URL on which the video can be displayed.
    pub url: String,
    /// Identification number as provided by the service the video originates from.
    pub id: String,
    /// Title of the video
    pub title: String,
    /// Text describing the video
    pub description: String,
    /// Length of the video in seconds
    pub length_seconds: u32,
    /// Keywords associated with the video as retrieved from the service
    pub keywords: Vec<String>,
    /// Identification number of the channel the video is associated with, may not be used depending on the service
    pub channel_id: String,
    /// How many views the service reported the video has. May not be used depending on the service
    pub views: u32,
    /// Watch status of the video, used to track if the video has been seen yet or not.
    pub status: VideoStatus,
    /// Text name of the author of the video
    pub author: String,
    /// Service which the video originates from
    pub source: VideoService,
    /// Identification for the thumbnail associated with the vide
    pub thumbnail: ContentIdentifier<Thumbnail>,
}

/// Data associated with video thumbnails
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Thumbnail {
    /// URL at which the thumbnail can be obtained
    pub url: String,
    /// Identifier of the thumbnail as provided by the service
    pub id: String,
    /// The actual image of the thumbnail.
    ///
    /// The value can be None if the thumbnail haven't been loaded yet, or if it haven't been downloaded.
    #[serde(skip)]
    image: Option<image::Handle>,
    /// Service the thumbnail originates from
    pub source: VideoService,
}

/// Convenience struct for tracking playlist updates
pub struct PlaylistFeed<'a> {
    /// The playlist
    pub playlist: &'a Playlist,
    /// Lates video of the playlist that haven't been marked as watched yet
    pub latest: &'a Video,
}
