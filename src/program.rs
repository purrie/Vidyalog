use serde::{Deserialize, Serialize};

use crate::{
    data::{Playlist, Thumbnail, Video},
    enums::{ColorTheme, WindowScreen},
    gui::Status,
    service::ServiceProvider,
};

mod database;
mod settings;
mod vidyalog;

#[derive(Default)]
pub struct Vidyalog {
    screen: WindowScreen,
    status: Status,
    web: ServiceProvider,
    data: Database,
    inputs: Inputs,
    settings: Settings,
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

#[derive(Serialize, Deserialize)]
pub struct Settings {
    theme: ColorTheme,
}
