use std::fmt::Display;

use super::TooltipText;

/// Display trait is used to convert the enum tags into text when passed to the tooltip widget
impl Display for TooltipText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TooltipText::OpenInBrowser => "Open in external browser",
            TooltipText::ChangeStatus => "Change video status",
            TooltipText::PlaylistDetails => "View videos of the playlist",
            TooltipText::ContinueWatching => "Open the next unfinished video in external browser",
            TooltipText::TrackPlaylist => "Track new videos of the playlist on the home page",
            TooltipText::UntrackPlaylist => "Stop tracking playlist updates",
            TooltipText::Delete => "Removes the playlist from the program",
        };
        write!(f, "{text}")
    }
}
