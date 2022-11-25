use iced::{
    widget::{
        button, column, container, horizontal_rule, row, scrollable, text, vertical_space, Column,
    },
    Element, Length,
};

use crate::{
    data::Playlist,
    enums::{Message, WindowScreen},
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
        let main = row!(
            column!(
                text(&self.title).width(Length::Fill),
                row!(text(format!("Video count: {}", self.video_count())).width(Length::Shrink))
            )
            .width(Length::Fill),
            button("Delete").on_press(Message::DeletePlaylist(self.id.clone())),
            button("Details").on_press(Message::OpenScreen(WindowScreen::PlaylistDetail(
                self.id.clone()
            ))),
            button("Open").on_press(Message::OpenInBrowser(self.url.clone()))
        )
        .width(Length::Fill)
        .spacing(5);

        // let col = column!(vertical_space(Length::Units(5)), main, horizontal_rule(1));
        let col = container(main).style(Styles::Box).padding(5);
        col.into()
    }
}

impl DetailView for Playlist {
    fn gui_detail_view(&self) -> Element<Message> {
        let top_box = Column::with_children(vec![
            text(&self.title).size(30).into(),
            horizontal_rule(2).into(),
            vertical_space(Length::Units(5)).into(),
            text(&self.description).into(),
        ])
        .padding(5);

        let content = container(top_box)
            .style(Styles::Header)
            .padding(5)
            .width(Length::Fill)
            .height(Length::Shrink);

        content.into()
    }
}
