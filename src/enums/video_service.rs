use std::fmt::Display;

use super::VideoService;

impl Display for VideoService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoService::Unknown => write!(f, "Unknown Service"),
            VideoService::Youtube => write!(f, "Youtube"),
        }
    }
}

impl VideoService {
    /// Used in serialization and deserialization, provides folder name where the files associated with the service should be kept
    pub fn get_path_name(&self) -> String {
        match self {
            VideoService::Unknown => "unknown",
            VideoService::Youtube => "youtube",
        }.to_string()
    }
}
