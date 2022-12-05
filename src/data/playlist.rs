use crate::{
    enums::ContentType,
    file::{File, PlaylistPath},
    service::{ContentID, ContentIdentifier},
};

use super::Playlist;

impl Playlist {
    /// Returns how many videos are in the playlist
    pub fn video_count(&self) -> usize {
        self.videos.len()
    }
    /// Updates the playlist with the information from the other playlist
    pub fn update(&mut self, other: Playlist) {
        if self.url != other.url {
            // TODO return an error
            return;
        }
        self.title = other.title;
        self.description = other.description;
        self.author = other.author;
        self.videos = other.videos;
    }
}

impl File for Playlist {
    type Path = PlaylistPath;
}
impl ContentID for Playlist {
    fn get_content_id(&self) -> ContentIdentifier<Self>
    where
        Self: Sized,
    {
        ContentIdentifier::new(&self.source, &self.id, ContentType::Playlist)
    }
}
