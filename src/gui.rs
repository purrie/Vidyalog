use iced::Element;

use crate::{enums::Message, program::Database};

mod playlist_gui;
mod styles;
mod video_gui;

/// The tuple for small size thumbnails, 0: x, 1: y
pub const THUMBNAIL_SIZE_SMALL: (u16, u16) = (168, 94);
/// The tuple for big size thumbnails, 0: x, 1: y
pub const THUMBNAIL_SIZE_BIG: (u16, u16) = (336, 188);

/// The trait is used for convenience of implementing a function that provides a list view of the data
pub trait ListView {
    fn gui_list_view<'a>(&self, database: &Database) -> Element<'a, Message>;
}
/// The trait is used for convenience of implementing a function that provides detailed information about the data
pub trait DetailView {
    fn gui_detail_view<'a>(&self, database: &Database) -> Element<'a, Message>;
}

/// Tags for customizing style of various iced widgets, they seamlessly convert into appropriate StyleSheets providing unified appearance to the controls.
pub enum Styles {
    Background,
    ContentFrame,
    Header,
    Danger,
}
