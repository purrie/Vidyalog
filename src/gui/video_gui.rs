use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text, Column},
    Alignment, Element, Length,
};

use crate::{data::Video, enums::Message, service::ContentID};

use super::{ListView, Styles};

impl ListView for Vec<&Video> {
    fn gui_list_view<'a>(&self) -> Element<'a, Message> {
        let ui = self.iter().map(|x| x.gui_list_view()).collect();
        let c = Column::with_children(ui).width(Length::Fill).spacing(4).padding(8);
        let c = row!(c, horizontal_space(Length::Units(4)));
        let scroll = scrollable(c).style(Styles::ContentFrame);
        let scroll = container(scroll)
            .height(Length::Fill)
            .style(Styles::Background);
        scroll.into()
    }
}

impl ListView for Video {
    fn gui_list_view<'a>(&self) -> Element<'a, Message> {
        let status = row!(
            text(format!("Video length: {}", self.get_length())),
            horizontal_space(Length::Units(10)),
            button(self.status.as_label())
                .on_press(Message::ToggleWatchStatus(self.get_content_id()))
                .style(Styles::ContentFrame.into()),
        )
        .align_items(Alignment::Center);
        let mini_info = row!(
            text(&self.author),
            horizontal_space(Length::Units(20)),
            status
        )
        .align_items(Alignment::Center);
        let info = column!(text(&self.title), mini_info).width(Length::Fill);
        let controls = button("Open").on_press(Message::OpenVideoExternally(self.get_content_id()));
        let line = row!(info, controls)
            .height(Length::Shrink)
            .width(Length::Fill);
        let line = container(line).style(Styles::ContentFrame).padding(5);
        line.into()
    }
}
