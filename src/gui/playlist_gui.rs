use iced::{
    widget::{
        button, column, container, horizontal_rule, horizontal_space, row, scrollable, text,
        tooltip, vertical_space, Column, Image,
    },
    Alignment, Element, Length,
};

use crate::{
    data::{Playlist, PlaylistFeed},
    enums::{Message, TooltipText, WindowScreen},
    program::Database,
    service::ContentID,
};

use super::{DetailView, ListView, Styles, THUMBNAIL_SIZE_BIG, THUMBNAIL_SIZE_SMALL};

impl ListView for Vec<Playlist> {
    /// UI interface for a list of playlists
    fn gui_list_view<'a>(&self, database: &Database) -> Element<'a, Message> {
        let list = Column::with_children(
            self.iter()
                .map(|x| x.gui_list_view(database))
                .map(Element::from)
                .collect(),
        )
        .width(Length::Fill)
        .spacing(4)
        .padding(8);
        let list = row!(list, horizontal_space(Length::Units(4)));
        let scroll = scrollable(list).style(Styles::ContentFrame);
        let cont = container(scroll)
            .style(Styles::Background)
            .height(Length::Fill)
            .width(Length::Fill);
        cont.into()
    }
}

impl<'p> ListView for Vec<PlaylistFeed<'p>> {
    /// UI for a list of playlist feeds, showing both a playlist and the first unwatched video in the playlist
    fn gui_list_view<'a>(&self, database: &Database) -> Element<'a, Message> {
        let list = Column::with_children(
            self.iter()
                .map(|x| x.gui_list_view(database))
                .map(Element::from)
                .collect(),
        )
        .width(Length::Fill)
        .spacing(4)
        .padding(8);
        let list = row!(list, horizontal_space(Length::Units(4)));
        let scroll = scrollable(list).style(Styles::ContentFrame);
        let cont = container(scroll)
            .style(Styles::Background)
            .height(Length::Fill)
            .width(Length::Fill);
        cont.into()
    }
}
impl<'p> ListView for PlaylistFeed<'p> {
    /// Creates a line UI displaying the playlist and the first unwatched video in the playlist
    fn gui_list_view<'a>(&self, database: &Database) -> Element<'a, Message> {
        let title = text(&self.playlist.title).size(20);
        let author = text(&self.playlist.author).size(16);

        let video_title = text(&self.latest.title);
        let video_length = text(format!("Video length: {}", self.latest.get_length()));
        let video_status = button(self.latest.status.as_label())
            .on_press(Message::ToggleWatchStatus(self.latest.get_content_id()))
            .style(Styles::ContentFrame.into());
        let video_status = tooltip(
            video_status,
            TooltipText::ChangeStatus,
            tooltip::Position::Right,
        );

        let mut video: Element<_> = column!(
            video_title,
            row!(
                video_length,
                horizontal_space(Length::Units(20)),
                video_status
            )
            .align_items(Alignment::Center)
        )
        .padding(4)
        .into();
        if let Some(th) = database.get_thumbnail_image(&self.latest.thumbnail) {
            let img = Image::new(th)
                .content_fit(iced::ContentFit::Fill)
                .width(Length::Units(THUMBNAIL_SIZE_SMALL.0))
                .height(Length::Units(THUMBNAIL_SIZE_SMALL.1));
            video = row!(img, video).padding(4).into();
        }
        video = column!(vertical_space(Length::Units(4)), video).into();
        let mut ui: Element<_> = column!(
            title,
            author,
            video,
        )
        .into();
        if let Some(th) = database.get_thumbnail_image(&self.playlist.thumbnail) {
            let img = Image::new(th)
                .content_fit(iced::ContentFit::Fill)
                .width(Length::Units(THUMBNAIL_SIZE_SMALL.0))
                .height(Length::Units(THUMBNAIL_SIZE_SMALL.1));
            ui = row!(img, ui).spacing(4).into();
        }
        let controls = column!(
            tooltip(
                button("Details").on_press(Message::OpenScreen(WindowScreen::PlaylistDetail(
                    self.playlist.get_content_id()
                ))),
                TooltipText::PlaylistDetails,
                tooltip::Position::Top
            )
            .style(Styles::Header),
            tooltip(
                button("Continue")
                    .on_press(Message::OpenVideoExternally(self.latest.get_content_id())),
                TooltipText::ContinueWatching,
                tooltip::Position::Bottom
            )
            .style(Styles::Header)
        )
        .spacing(4);
        let content = container(
            row!(ui, horizontal_space(Length::Fill), controls).align_items(Alignment::Center),
        )
        .padding(5)
        .style(Styles::ContentFrame)
        .width(Length::Fill)
        .height(Length::Shrink);

        content.into()
    }
}
impl ListView for Playlist {
    /// UI displaying information about the playlist in a line
    fn gui_list_view<'a>(&self, database: &Database) -> Element<'a, Message> {
        let info = column!(
            text(&self.title).size(20),
            text(&self.author).size(16),
            vertical_space(Length::Units(4)),
            text(format!("Video count: {}", self.video_count())).size(16)
        )
        .width(Length::Fill);

        let controls = row!(
            tooltip(
                button(if self.tracked { "Untrack" } else { "Track" })
                    .on_press(Message::ToggleTracking(self.get_content_id())),
                if self.tracked {
                    TooltipText::UntrackPlaylist
                } else {
                    TooltipText::TrackPlaylist
                },
                tooltip::Position::Bottom
            )
            .style(Styles::Header),
            tooltip(
                button("Delete")
                    .on_press(Message::DeletePlaylist(self.get_content_id()))
                    .style(Styles::Danger.into()),
                TooltipText::Delete,
                tooltip::Position::Bottom
            )
            .style(Styles::Header),
            tooltip(
                button("Details").on_press(Message::OpenScreen(WindowScreen::PlaylistDetail(
                    self.get_content_id()
                ))),
                TooltipText::PlaylistDetails,
                tooltip::Position::Bottom
            )
            .style(Styles::Header),
            tooltip(
                button("Open").on_press(Message::OpenInBrowser(self.url.clone())),
                TooltipText::OpenInBrowser,
                tooltip::Position::Bottom
            )
            .style(Styles::Header)
        )
        .spacing(4);

        let mut main = row!().width(Length::Fill).height(Length::Shrink).spacing(5);
        if let Some(th) = database.get_thumbnail_image(&self.thumbnail) {
            let img = Image::new(th)
                .content_fit(iced::ContentFit::Fill)
                .height(Length::Units(THUMBNAIL_SIZE_SMALL.1))
                .width(Length::Units(THUMBNAIL_SIZE_SMALL.0));
            main = main.push(img);
        }
        let main = main.push(row!(info, controls).align_items(Alignment::Center));

        // let col = column!(vertical_space(Length::Units(5)), main, horizontal_rule(1));
        let col = container(main).style(Styles::ContentFrame).padding(5);
        col.into()
    }
}

impl DetailView for Playlist {
    /// Creates UI for the playlist header for the detail screen
    fn gui_detail_view<'a>(&self, database: &Database) -> Element<'a, Message> {
        let top_box = column!(
            text(&self.title).size(30),
            horizontal_rule(4).style(Styles::Header),
            text(format!("by {}", &self.author)),
            vertical_space(Length::Units(5)),
            text(&self.description)
        )
        .padding(5)
        .width(Length::Fill);
        let controls = column!(
            tooltip(
                button("Open").on_press(Message::OpenInBrowser(self.url.clone())),
                TooltipText::OpenInBrowser,
                tooltip::Position::Left
            ),
            tooltip(
                button(if self.tracked { "Untrack" } else { "Track" })
                    .on_press(Message::ToggleTracking(self.get_content_id())),
                if self.tracked {
                    TooltipText::UntrackPlaylist
                } else {
                    TooltipText::TrackPlaylist
                },
                tooltip::Position::Left
            )
        )
        .spacing(4)
        .align_items(Alignment::End);
        let mut content = row!(top_box, controls,).spacing(4);

        if let Some(th) = database.get_thumbnail_image(&self.thumbnail) {
            let img = Image::new(th)
                .content_fit(iced::ContentFit::Fill)
                .height(Length::Units(THUMBNAIL_SIZE_BIG.1))
                .width(Length::Units(THUMBNAIL_SIZE_BIG.0));
            content = content.push(img);
        }

        let content = container(content)
            .style(Styles::Header)
            .padding(5)
            .width(Length::Fill)
            .height(Length::Shrink);

        content.into()
    }
}
