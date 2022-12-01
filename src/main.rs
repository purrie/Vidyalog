use program::Vidyalog;
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
mod service;
mod web;
mod program;
mod icons;

fn main() -> iced::Result {
    Vidyalog::run(Settings::default())
}
