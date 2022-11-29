use iced::{
    widget::{button, column, container, row, text, text_input},
    Application, Command, Element, Length, Theme,
};

use crate::{
    data::Playlist,
    enums::{Message, VideoStatus, WindowScreen},
    file::File,
    gui::{DetailView, ListView, Styles},
    service::ContentIdentifier,
};

use super::Vidyalog;

impl Application for Vidyalog {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = Vidyalog::default();
        let updates: Vec<_> = app
            .data
            .playlists
            .iter()
            .map(|x| {
                Command::perform(
                    x.source.get_playlist(app.web.get_browser(), x.url.clone()),
                    Message::AddPlaylist,
                )
            })
            .collect();
        let comm = Command::batch(updates);
        (app, comm)
    }

    fn title(&self) -> String {
        "Vidyalog".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::AddPlaylistURL(url) => {
                self.inputs.add_playlist = url;
                Command::none()
            }
            Message::OpenInBrowser(url) => {
                open::that(url).unwrap();
                Command::none()
            }
            Message::OpenScreen(screen) => {
                self.screen = screen;
                Command::none()
            }
            Message::QueryPlaylist => {
                let p = std::mem::take(&mut self.inputs.add_playlist);
                Command::perform(self.web.get_playlist(p), Message::AddPlaylist)
            }
            Message::AddPlaylist(Ok(playlist)) => {
                let comm = match self.data.get_missing_videos(&playlist.videos) {
                    Some(miss) => {
                        let missing: Vec<Command<_>> = miss
                            .iter()
                            .map(|x| {
                                Command::perform(
                                    playlist
                                        .source
                                        .get_video(self.web.get_browser(), x.get_url()),
                                    Message::AddVideo,
                                )
                            })
                            .collect();
                        Command::batch(missing)
                    }
                    None => Command::none(),
                };
                if let Err(e) = self.data.add_playlist(playlist) {
                    self.status
                        .report(format!("Failed to save playlist: {}", e));
                    return Command::none();
                } else {
                    return comm;
                }
            }
            Message::DeletePlaylist(id) => {
                if let Err(e) = self.data.remove_playlist(&id) {
                    self.status
                        .report(format!("Failed to remove playlist: {}", e));
                } else {
                    self.status.report(format!("Removed playlist"));
                }
                Command::none()
            }
            Message::AddPlaylist(Err(err)) => {
                println!("{err}");
                self.status.report(format!("{err}"));
                Command::none()
            }
            Message::AddVideo(Ok(video)) => {
                if let Err(e) = self.data.add_video(video) {
                    self.status.report(format!("Failed to add or update video: {}", e));
                }
                Command::none()
            }
            Message::AddVideo(Err(e)) => {
                println!("{e}");
                self.status.report(format!("{e}"));
                Command::none()
            }
            Message::OpenVideoExternally(id) => {
                let Some(mut vid) = self.data.get_video_mut(&id) else {
                    self.status.report(format!("Video {} not found", id));
                    return Command::none();
                };
                if vid.status == VideoStatus::Unseen {
                    vid.status = VideoStatus::Browsed;
                }
                open::that(&vid.url).unwrap();
                if let Err(e) = vid.save() {
                    self.status.report(format!(
                        "Failed to update video {} because {}",
                        vid.title, e
                    ));
                }
                Command::none()
            }
            Message::ToggleWatchStatus(id) => {
                let Some(mut vid) = self.data.get_video_mut(&id) else {
                    self.status.report(format!("Video {} not found", id));
                    return Command::none();
                };
                vid.status = match vid.status {
                    VideoStatus::Unseen => VideoStatus::Watched,
                    VideoStatus::Browsed => VideoStatus::Watched,
                    VideoStatus::Watched => VideoStatus::Unseen,
                };
                if let Err(e) = vid.save() {
                    self.status.report(format!(
                        "Failed to update video {} because {}",
                        vid.title, e
                    ));
                }
                Command::none()
            }
            Message::SetTheme(t) => {
                self.theme = t;
                Command::none()
            }
            Message::ToggleTracking(id) => {
                let Some(mut pl) = self.data.get_playlist_mut(&id) else {
                    self.status.report(format!("Couldn't retrieve playlist {}", id));
                    return Command::none();
                };
                pl.tracked = !pl.tracked;
                if let Err(e) = pl.save() {
                    self.status
                        .report(format!("Failed to update video {} because {}", pl.title, e));
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let bar = self.side_bar_view();

        let main = match &self.screen {
            WindowScreen::PlaylistTracker => self.playlist_tracker_view(),
            WindowScreen::PlaylistDetail(id) => self.playlist_detail_view(id),
            WindowScreen::Home => self.home_view(),
        };

        let content = column![row![bar, main].height(Length::Fill), self.status.line()];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}

impl Vidyalog {
    fn side_bar_view(&self) -> Element<Message> {
        let buttons = column![
            button("Home").on_press(Message::OpenScreen(WindowScreen::Home)),
            button("Playlists").on_press(Message::OpenScreen(WindowScreen::PlaylistTracker)),
            button("Theme").on_press({
                if self.theme == Theme::Light {
                    Message::SetTheme(Theme::Dark)
                } else {
                    Message::SetTheme(Theme::Light)
                }
            })
        ];
        let content = container(buttons)
            .style(Styles::Header)
            .width(Length::Shrink)
            .height(Length::Fill)
            .padding(5);
        content.into()
    }
    fn home_view(&self) -> Element<Message> {
        let pl = self.data.get_fresh_playlists();
        if pl.len() == 0 {
            let tex: Element<_> = text("No new videos to watch").into();
            return container(tex)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(Styles::Background)
                .center_x()
                .center_y()
                .into();
        }
        let list = pl.gui_list_view();
        list.into()
    }
    fn playlist_tracker_view(&self) -> Element<Message> {
        let bar = row!(
            text_input(
                "Add Playlist URL",
                &self.inputs.add_playlist,
                Message::AddPlaylistURL
            ),
            button("Add").on_press(Message::QueryPlaylist)
        );
        let list = self.data.playlists.gui_list_view();
        column!(bar, list).into()
    }
    fn playlist_detail_view(&self, id: &ContentIdentifier<Playlist>) -> Element<Message> {
        let Some(pl) = self.data.get_playlist(id) else {
            return text(format!("Playlist id doesn't exist: {}", id)).into();
        };
        let vids = self.data.get_videos_by_id(&pl.videos);

        let detail = pl.gui_detail_view();
        let video_list = vids.gui_list_view();
        column!(detail, video_list).into()
    }
}
