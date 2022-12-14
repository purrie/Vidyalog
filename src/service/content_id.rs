use std::{fmt::Display, marker::PhantomData};

use crate::enums::{ContentType, VideoService};

use super::{ContentID, ContentIdentifier};

impl<T> ContentIdentifier<T> {
    /// Creates a new identifier
    pub fn new(service: &VideoService, id: &str, content: ContentType) -> Self {
        ContentIdentifier {
            service: service.clone(),
            id: id.to_string(),
            content,
            pd: PhantomData,
        }
    }
    /// Tests if the content matches this identifier
    pub fn identify(&self, other: &T) -> bool
    where
        T: ContentID,
    {
        let oc = other.get_content_id();
        oc == *self
    }
    /// Returns an URL that points to the content
    ///
    /// # Error
    /// If the identifier isn't properly created, it will return "Invalid content type" instead
    pub fn get_url(&self) -> String {
        match &self.content {
            ContentType::Playlist => self.service.get_playlist_url(&self.id),
            ContentType::Video => self.service.get_video_url(&self.id),
            ContentType::Thumbnail => self.service.get_thumbnail_url(&self.id),
            ContentType::Invalid => String::from("Invalid content type"),
        }
    }
}

impl<T> Display for ContentIdentifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.service, self.id)
    }
}
