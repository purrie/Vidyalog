use iced::{
    widget::{
        button, column, container, horizontal_rule, horizontal_space, row, scrollable, text,
        vertical_space, Column,
    },
    Alignment, Element, Length,
};

use crate::{
    data::{Playlist, PlaylistFeed},
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
    fn gui_list_view<'a>(&self) -> Element<'a, Message> {
        let list = Column::with_children(
            self.iter()
                .map(|x| x.gui_list_view())
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
    fn gui_list_view<'a>(&self) -> Element<'a, Message> {
        let title = text(&self.playlist.title).size(20);
        let author = text(&self.playlist.author).size(16);

        let video_title = text(&self.latest.title);
        let video_length = text(format!("Video length: {}", self.latest.get_length()));
        let video_status = button(self.latest.status.as_label())
            .on_press(Message::ToggleWatchStatus(self.latest.get_content_id()))
            .style(Styles::ContentFrame.into());

        let ui = column!(
            title,
            row!(
                author,
                horizontal_space(Length::Units(20)),
                column!(
                    vertical_space(Length::Units(4)),
                    video_title,
                    row!(
                        video_length,
                        horizontal_space(Length::Units(20)),
                        video_status
                    )
                    .align_items(Alignment::Center)
                )
            )
        )
        .width(Length::Fill);
        let controls = row!(
            button("Details").on_press(Message::OpenScreen(WindowScreen::PlaylistDetail(
                self.playlist.get_content_id()
            ))),
            button("Continue").on_press(Message::OpenVideoExternally(self.latest.get_content_id()))
        )
        .spacing(4);
        let content = container(row!(ui, controls).align_items(Alignment::Center))
            .padding(5)
            .style(Styles::ContentFrame)
            .width(Length::Fill)
            .height(Length::Shrink);

        content.into()
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
            button(if self.tracked { "Untrack" } else { "Track" })
                .on_press(Message::ToggleTracking(self.get_content_id())),
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
        let col = container(main).style(Styles::ContentFrame).padding(5);
        col.into()
    }
}

impl DetailView for Playlist {
    fn gui_detail_view<'a>(&self) -> Element<'a, Message> {
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
            button("Open").on_press(Message::OpenInBrowser(self.url.clone())),
            button(if self.tracked { "Untrack" } else { "Track" })
                .on_press(Message::ToggleTracking(self.get_content_id()))
        )
        .spacing(4)
        .align_items(Alignment::End);
        let content = row!(top_box, controls,);

        let content = container(content)
            .style(Styles::Header)
            .padding(5)
            .width(Length::Fill)
            .height(Length::Shrink);

        content.into()
    }
}
