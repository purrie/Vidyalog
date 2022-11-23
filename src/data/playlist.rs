use crate::file::{File, FileID, PlaylistPath};

use super::Playlist;

impl Playlist {
    pub fn video_count(&self) -> usize {
        self.videos.len()
    }
}

impl File for Playlist {
    type Path = PlaylistPath;
}
impl FileID for Playlist {
    fn get_file_id(&self) -> &str {
        &self.id
    }
}
