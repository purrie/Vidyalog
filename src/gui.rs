use crate::{data::Playlist, enums::WindowScreen, service::ServiceProvider};

mod playlist_tracker;
mod status;
mod window;

#[derive(Default)]
pub struct Window {
    screen: WindowScreen,
    status: Status,
    web: ServiceProvider,

    playlist_tracker: PlaylistTracker,
}

#[derive(Default)]
pub struct PlaylistTracker {
    add_url: String,
    playlists: Vec<Playlist>,
}

#[derive(Default)]
pub struct Status {
    current: String,
}
