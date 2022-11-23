use gui::Window;
use iced::{Application, Settings};

extern crate dirs;
extern crate iced;
extern crate regex;
extern crate reqwest;
extern crate ron;
extern crate serde;
extern crate htmlentity;
extern crate open;

mod data;
mod enums;
mod file;
mod gui;
mod paths;
mod service;
mod web;

fn main() -> iced::Result {
    Window::run(Settings::default())
}
