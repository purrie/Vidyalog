use crate::{
    file::{File, PlaylistPath},
    service::{ContentID, ContentIdentifier},
};

use super::Playlist;

impl Playlist {
    pub fn video_count(&self) -> usize {
        self.videos.len()
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
