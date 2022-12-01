use std::fmt::Display;

use super::TooltipText;

impl Display for TooltipText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TooltipText::OpenInBrowser => write!(f, "Open in external browser"),
            TooltipText::ChangeStatus => write!(f, "Change video status"),
            TooltipText::PlaylistDetails => write!(f, "View videos of the playlist"),
            TooltipText::ContinueWatching => {
                write!(f, "Open the next unfinished video in external browser")
            }
            TooltipText::TrackPlaylist => {
                write!(f, "Track new videos of the playlist on the home page")
            }
            TooltipText::UntrackPlaylist => write!(f, "Stop tracking playlist updates"),
            TooltipText::Delete => write!(f, "Removes the playlist from the program"),
        }
    }
}
