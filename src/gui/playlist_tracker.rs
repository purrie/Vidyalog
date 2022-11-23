use iced::{
    widget::{
        button, column, horizontal_rule, row, scrollable, text, text_input, vertical_space, Column,
    },
    Element, Length,
};

use crate::{data::Playlist, enums::Message};

use super::PlaylistTracker;

impl PlaylistTracker {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let add_bar = row!(
            text_input(
                "Add new playlist url",
                &self.add_url,
                Message::AddPlaylistURL
            )
            .width(Length::Fill),
            button("Add")
                .on_press(Message::AddPlaylist)
                .width(Length::Shrink)
        )
        .height(Length::Shrink);
        let list = Column::with_children(
            self.playlists
                .iter()
                .map(|x| playlist_line_view(x))
                .map(Element::from)
                .collect(),
        );
        let scroll = scrollable(list).height(Length::Fill);
        let content = column!(add_bar, scroll);

        content.into()
    }

    pub fn add_playlist(&mut self, playlist: Playlist) {
        if let Some(f) = self.playlists.iter().position(|x| x.id == playlist.id) {
            self.playlists.remove(f);
        }
        self.playlists.push(playlist);
    }
    pub fn remove(&mut self, id: &str) -> Option<Playlist> {
        if let Some(i) = self.playlists.iter().position(|x| x.id == id) {
            return Some(self.playlists.remove(i));
        }
        None
    }
}

fn playlist_line_view<'a>(playlist: &Playlist) -> Element<'a, Message> {
    let main = row!(
        column!(
            text(&playlist.title).width(Length::Fill),
            row!(text(format!("Video count: {}", playlist.video_count())).width(Length::Shrink))
        )
        .width(Length::Fill),
        button("Delete").on_press(Message::DeletePlaylist(playlist.id.clone())),
        button("Mark as watched"),
        button("Track on home page"),
        button("Open").on_press(Message::OpenInBrowser(playlist.url.clone()))
    )
    .width(Length::Fill)
    .spacing(5);

    column!(vertical_space(Length::Units(5)), main, horizontal_rule(1)).into()
}
