use crate::file::{File, FileID, VideoPath};

use super::Video;

impl File for Video {
    type Path = VideoPath;
}
impl FileID for Video {
    fn get_file_id(&self) -> &str {
        &self.id
    }
}

impl Video {
    pub fn get_length(&self) -> String {
        let m_s = self.length_seconds % 60;
        let m_m = self.length_seconds / 60;
        let m_h = m_m / 60;
        let m_m = m_m % 60;

        if m_h > 0 {
            format!("{}:{}:{}", m_h, m_m, m_s)
        } else {
            format!("{}:{}", m_m, m_s)
        }
    }
}
