use serde::{Deserialize, Serialize};

mod playlist;

/// Contains all the information necessary for tracking and playing a playlist
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Playlist {
    pub url: String,
    pub id: String,
    pub title: String,
    /// Contains ID numbers for the videos
    pub videos: Vec<String>,
}
