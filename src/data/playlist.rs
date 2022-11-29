use crate::{
    file::{File, PlaylistPath},
    service::{ContentID, ContentIdentifier},
};

use super::Playlist;

impl Playlist {
    pub fn video_count(&self) -> usize {
        self.videos.len()
    }
    pub fn update(&mut self, other: Playlist) {
        if self.url != other.url {
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
        ContentIdentifier::new(&self.source, &self.id)
    }
}
