use crate::file::{File, VideoPath, FileID};

use super::Video;


impl File for Video {
    type Path = VideoPath;
}
impl FileID for Video {
    fn get_file_id(&self) -> &str {
        &self.id
    }
}
