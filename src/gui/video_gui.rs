use iced::{
    widget::{button, column, container, row, scrollable, text, Column},
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
        let info = column!(text(&self.title), text(&self.author)).width(Length::Fill);
        let controls = button("Open").on_press(Message::OpenInBrowser(self.url.clone()));
        let line = row!(info, controls)
            .height(Length::Shrink)
            .width(Length::Fill);
        let line = container(line).style(Styles::Distinguished).padding(5);
        line.into()
    }
}
