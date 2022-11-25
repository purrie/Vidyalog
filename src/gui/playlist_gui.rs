use iced::{
    widget::{
        button, column, container, horizontal_rule, row, scrollable, text, vertical_space, Column,
    },
    Alignment, Element, Length,
};

use crate::{
    data::Playlist,
    enums::{Message, WindowScreen},
    service::ContentID,
};

use super::{DetailView, ListView, Styles};

impl ListView for Vec<Playlist> {
    fn gui_list_view<'a>(&self) -> Element<'a, Message> {
        let list = Column::with_children(
            self.iter()
                .map(|x| x.gui_list_view())
                .map(Element::from)
                .collect(),
        )
        .padding(5);
        let scroll = scrollable(list).height(Length::Fill);

        scroll.into()
    }
}

impl ListView for Playlist {
    fn gui_list_view<'a>(&self) -> Element<'a, Message> {
        let info = column!(
            text(&self.title).size(20),
            text(&self.author).size(16),
            text(format!("Video count: {}", self.video_count())).size(16)
        )
        .width(Length::Fill);

        let controls = row!(
            button("Delete")
                .on_press(Message::DeletePlaylist(self.get_content_id()))
                .style(Styles::Danger.into()),
            button("Details").on_press(Message::OpenScreen(WindowScreen::PlaylistDetail(
                self.get_content_id()
            ))),
            button("Open").on_press(Message::OpenInBrowser(self.url.clone()))
        )
        .spacing(4);

        let main = row!(info, controls,)
            .width(Length::Fill)
            .spacing(5)
            .align_items(Alignment::Center);

        // let col = column!(vertical_space(Length::Units(5)), main, horizontal_rule(1));
        let col = container(main).style(Styles::Distinguished).padding(5);
        col.into()
    }
}

impl DetailView for Playlist {
    fn gui_detail_view(&self) -> Element<Message> {
        let top_box = column!(
            text(&self.title).size(30),
            horizontal_rule(4).style(Styles::Header),
            text(format!("by {}", &self.author)),
            vertical_space(Length::Units(5)),
            text(&self.description)
        )
        .padding(5)
        .width(Length::Fill);
        let controls = column!(button("Open").on_press(Message::OpenInBrowser(self.url.clone())));
        let content = row!(top_box, controls,);

        let content = container(content)
            .style(Styles::Header)
            .padding(5)
            .width(Length::Fill)
            .height(Length::Shrink);

        content.into()
    }
}
