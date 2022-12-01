use iced::Theme;

use crate::{
    data::{Playlist, Thumbnail, Video},
    enums::WindowScreen,
    gui::Status,
    service::ServiceProvider,
};

mod database;
mod vidyalog;

#[derive(Default)]
pub struct Vidyalog {
    screen: WindowScreen,
    status: Status,
    web: ServiceProvider,
    data: Database,
    inputs: Inputs,
    theme: Theme,
}

pub struct Database {
    playlists: Vec<Playlist>,
    videos: Vec<Video>,
    thumbnails: Vec<Thumbnail>,
}

#[derive(Default)]
pub struct Inputs {
    add_playlist: String,
}
