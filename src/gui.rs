use iced::Element;

use crate::{enums::Message, program::Database};

mod playlist_gui;
mod status;
mod styles;
mod video_gui;

pub const THUMBNAIL_SIZE_SMALL: (u16, u16) = (168, 94);
pub const THUMBNAIL_SIZE_BIG: (u16, u16) = (336, 188);

#[derive(Default)]
pub struct Status {
    current: String,
}

pub trait ListView {
    fn gui_list_view<'a>(&self, database: &Database) -> Element<'a, Message>;
}
pub trait DetailView {
    fn gui_detail_view<'a>(&self, database: &Database) -> Element<'a, Message>;
}

pub enum Styles {
    Background,
    ContentFrame,
    Header,
    Danger,
}
