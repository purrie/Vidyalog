use iced::{
    widget::{
        button, column, container, horizontal_space, image, row, scrollable, text, vertical_space,
        Column,
    },
    Alignment, Element, Length,
};

use crate::{data::Video, enums::Message, program::Database, service::ContentID};

use super::{ListView, Styles, THUMBNAIL_SIZE_SMALL};

impl ListView for Vec<&Video> {
    fn gui_list_view<'a>(&self, database: &Database) -> Element<'a, Message> {
        let ui = self.iter().map(|x| x.gui_list_view(database)).collect();
        let c = Column::with_children(ui)
            .width(Length::Fill)
            .height(Length::Shrink)
            .spacing(4)
            .padding(8);
        let c = row!(c, horizontal_space(Length::Units(4)));
        let scroll = scrollable(c)
            .height(Length::Fill)
            .style(Styles::ContentFrame);
        let scroll = container(scroll)
            .height(Length::Fill)
            .style(Styles::Background);
        scroll.into()
    }
}

impl ListView for Video {
    fn gui_list_view<'a>(&self, database: &Database) -> Element<'a, Message> {
        let status = row!(
            text(format!("Video length: {}", self.get_length())),
            horizontal_space(Length::Units(10)),
            button(self.status.as_label())
                .on_press(Message::ToggleWatchStatus(self.get_content_id()))
                .style(Styles::ContentFrame.into()),
        )
        .align_items(Alignment::Center);
        let info = column!(
            text(&self.title),
            text(&self.author),
            vertical_space(Length::Units(4)),
            status
        )
        .width(Length::Fill);
        let controls = button("Open").on_press(Message::OpenVideoExternally(self.get_content_id()));
        let mut line = row!().spacing(4).height(Length::Shrink).width(Length::Fill);
        if let Some(th) = database.get_thumbnail_image(&self.thumbnail) {
            let img = image::Image::new(th)
                .content_fit(iced::ContentFit::Fill)
                .height(Length::Units(THUMBNAIL_SIZE_SMALL.1))
                .width(Length::Units(THUMBNAIL_SIZE_SMALL.0));
            line = line.push(img);
        }
        line = line.push(info).push(controls);
        let line = container(line).style(Styles::ContentFrame).padding(5);
        line.into()
    }
}
