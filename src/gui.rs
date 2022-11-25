use iced::Element;

use crate::enums::Message;

mod playlist_gui;
mod video_gui;
mod status;
mod styles;
mod colors;

#[derive(Default)]
pub struct Status {
    current: String,
}

pub trait ListView {
    fn gui_list_view<'a>(&self) -> Element<'a, Message>;
}
pub trait DetailView {
    fn gui_detail_view(&self) -> Element<Message>;
}

pub enum Styles {
    Box,
    Header,
}

pub trait StyleAdjustment {
    fn lighter(self) -> Self;
    fn darker(self) -> Self;
}
