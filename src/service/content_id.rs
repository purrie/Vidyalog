use std::{fmt::Display, marker::PhantomData};

use crate::enums::VideoService;

use super::{ContentID, ContentIdentifier};

impl<T> ContentIdentifier<T> {
    pub fn new(service: &VideoService, id: &str) -> Self {
        ContentIdentifier {
            service: service.clone(),
            id: id.to_string(),
            pd: PhantomData,
        }
    }
    pub fn identify(&self, other: &T) -> bool where T: ContentID {
        let oc = other.get_content_id();
        oc == *self
    }
}

impl<T> Display for ContentIdentifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.service, self.id)
    }
}

impl<T> ContentIdentifier<T> {
    pub fn get_url(&self) -> String {
        self.service.get_video_url(&self.id)
    }
}
