use iced::{
    widget::{row, text},
    Element,
};

use crate::enums::Message;

use super::Status;

impl Status {
    /// Provides UI for the status line
    pub fn line<'a>(&self) -> Element<'a, Message> {
        row!(text(&self.current))
            .height(iced::Length::Shrink)
            .into()
    }
    /// Adds a new message to the status
    pub fn report(&mut self, status: String) {
        self.current = status;
    }
}
