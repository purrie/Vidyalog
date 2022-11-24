use iced::Element;

use crate::enums::Message;

mod playlist_gui;
mod video_gui;
mod status;

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
