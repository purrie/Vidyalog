use crate::{
    file::{File, VideoPath},
    service::{ContentID, ContentIdentifier}, enums::ContentType,
};

use super::Video;

impl File for Video {
    type Path = VideoPath;
}
impl ContentID for Video {
    fn get_content_id(&self) -> ContentIdentifier<Self>
    where
        Self: Sized,
    {
        ContentIdentifier::new(&self.source, &self.id, ContentType::Video)
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
    pub fn update(&mut self, other: Video) {
        if self.url != other.url {
            return;
        }
        self.title = other.title;
        self.description = other.description;
        self.length_seconds = other.length_seconds;
        self.keywords = other.keywords;
        self.channel_id = other.channel_id;
        self.views = other.views;
        self.author = other.author;
    }
}
