use serde::{Deserialize, Serialize};

use crate::{
    enums::{VideoService, VideoStatus},
    service::ContentIdentifier,
};

mod playlist;
mod video;

/// Contains all the information necessary for tracking and playing a playlist
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub struct Playlist {
    pub url: String,
    pub id: String,
    pub title: String,
    pub description: String,
    pub author: String,
    /// Contains ID numbers for the videos
    pub videos: Vec<ContentIdentifier<Video>>,
    pub source: VideoService,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub struct Video {
    pub url: String,
    pub id: String,
    pub title: String,
    pub description: String,
    pub length_seconds: u32,
    pub keywords: Vec<String>,
    pub channel_id: String,
    pub views: u32,
    pub status: VideoStatus,
    pub author: String,
    pub source: VideoService,
}
