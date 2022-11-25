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
