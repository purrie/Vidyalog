use iced::{
    widget::{button, column, container, row, scrollable, text, Column, horizontal_space},
    Element, Length,
};

use crate::{data::Video, enums::Message};

use super::{ListView, Styles};

impl ListView for Vec<&Video> {
    fn gui_list_view<'a>(&self) -> Element<'a, Message> {
        let ui = self.iter().map(|x| x.gui_list_view()).collect();
        let c = Column::with_children(ui).width(Length::Fill);
        scrollable(c).height(Length::Fill).into()
    }
}

impl ListView for Video {
    fn gui_list_view<'a>(&self) -> Element<'a, Message> {
        let mini_info = row!(
            text(&self.author),
            horizontal_space(Length::Units(20)),
            text(format!(
                "Video length: {}",
                self.get_length()
            ))
        );
        let info = column!(text(&self.title), mini_info).width(Length::Fill);
        let controls = button("Open").on_press(Message::OpenInBrowser(self.url.clone()));
        let line = row!(info, controls)
            .height(Length::Shrink)
            .width(Length::Fill);
        let line = container(line).style(Styles::Distinguished).padding(5);
        line.into()
    }
}
