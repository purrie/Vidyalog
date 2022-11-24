use iced::{
    widget::{button, column, row, scrollable, text, Column},
    Element, Length,
};

use crate::{data::Video, enums::Message};

use super::ListView;

impl ListView for Vec<&Video> {
    fn gui_list_view<'a>(&self) -> Element<'a, Message> {
        let ui = self.iter().map(|x| x.gui_list_view()).collect();
        let c = Column::with_children(ui).width(Length::Fill);
        scrollable(c).height(Length::Fill).into()
    }
}

impl ListView for Video {
    fn gui_list_view<'a>(&self) -> Element<'a, Message> {
        row!(column!(
            text(&self.title),
            row!(text(&self.author), button("Open"))
        ))
        .width(Length::Fill)
        .height(Length::Shrink)
        .into()
    }
}
