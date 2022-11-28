use std::fmt::Display;

use super::VideoStatus;

impl Display for VideoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoStatus::Unseen => write!(f, "Unseen"),
            VideoStatus::Browsed => write!(f, "Opened"),
            VideoStatus::Watched => write!(f, "Watched"),
        }
    }
}

impl VideoStatus {
    pub fn as_label(&self) -> &'static str {
        match self {
            VideoStatus::Unseen => "Unseen",
            VideoStatus::Browsed => "Opened",
            VideoStatus::Watched => "Watched",
        }
    }
}
