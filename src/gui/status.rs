use iced::{
    widget::{row, text},
    Element,
};

use crate::enums::Message;

use super::Status;

impl Status {
    pub fn line<'a>(&self) -> Element<'a, Message> {
        row!(text(&self.current))
            .height(iced::Length::Shrink)
            .into()
    }
    pub fn report(&mut self, status: String) {
        self.current = status;
    }
}
