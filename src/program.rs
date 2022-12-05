use serde::{Deserialize, Serialize};

use crate::{
    data::{Playlist, Thumbnail, Video},
    enums::{ColorTheme, WindowScreen},
    service::ServiceProvider,
};

mod database;
mod settings;
mod vidyalog;
mod status;

/// This is the main program data that implements all the behaviors of the program.
#[derive(Default)]
pub struct Vidyalog {
    /// Currently displayed screen
    screen: WindowScreen,
    /// Status line of the program
    status: Status,
    /// Web services, used for retrieving data from the internet
    web: ServiceProvider,
    /// All the program data
    data: Database,
    /// All the widgets used as inputs of user informations
    inputs: Inputs,
    /// Program preferences
    settings: Settings,
}

/// The database is designed to hold all the data about videos, playlists and other associated items.
pub struct Database {
    playlists: Vec<Playlist>,
    videos: Vec<Video>,
    thumbnails: Vec<Thumbnail>,
}

/// Inputs holds the data of any input field and other controls for user customization and functionality.
#[derive(Default)]
pub struct Inputs {
    add_playlist: String,
}

/// Settings struct, holds informations about prefered user appearance and behavior of the program.
#[derive(Serialize, Deserialize)]
pub struct Settings {
    theme: ColorTheme,
}

/// Status line of the program, it is used to convey information and feedback about what's happening to the user.
#[derive(Default)]
pub struct Status {
    current: String,
}
