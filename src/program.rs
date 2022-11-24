use crate::{
    data::{Playlist, Video},
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
}

pub struct Database {
    playlists: Vec<Playlist>,
    videos: Vec<Video>,
}

#[derive(Default)]
pub struct Inputs {
    add_playlist: String,
}
