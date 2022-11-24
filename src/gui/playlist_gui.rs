use iced::{
    widget::{button, column, horizontal_rule, row, scrollable, text, vertical_space, Column},
    Element, Length,
};

use crate::{
    data::Playlist,
    enums::{Message, WindowScreen},
};

use super::{DetailView, ListView};

fn playlist_line_view<'a>(playlist: &Playlist) -> Element<'a, Message> {
    let main = row!(
        column!(
            text(&playlist.title).width(Length::Fill),
            row!(text(format!("Video count: {}", playlist.video_count())).width(Length::Shrink))
        )
        .width(Length::Fill),
        button("Delete").on_press(Message::DeletePlaylist(playlist.id.clone())),
        button("Details").on_press(Message::OpenScreen(WindowScreen::PlaylistDetail(
            playlist.id.clone()
        ))),
        button("Open").on_press(Message::OpenInBrowser(playlist.url.clone()))
    )
    .width(Length::Fill)
    .spacing(5);

    column!(vertical_space(Length::Units(5)), main, horizontal_rule(1)).into()
}

impl ListView for Vec<Playlist> {
    fn gui_list_view<'a>(&self) -> Element<'a, Message> {
        let list = Column::with_children(
            self.iter()
                .map(|x| playlist_line_view(x))
                .map(Element::from)
                .collect(),
        );
        let scroll = scrollable(list).height(Length::Fill);

        scroll.into()
    }
}

impl DetailView for Playlist {
    fn gui_detail_view(&self) -> Element<Message> {
        column!(row!(text(&self.title)), row!(text(&self.id)),)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into()
    }
}
