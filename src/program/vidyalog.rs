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
                    Message::UpdatePlaylist,
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
            Message::AddPlaylist => {
                let p = std::mem::take(&mut self.inputs.add_playlist);
                Command::perform(self.web.get_playlist(p), Message::ResultPlaylist)
            }
            Message::ResultPlaylist(Ok(playlist)) => {
                let comm = match self.data.get_missing_videos(&playlist.videos) {
                    Some(miss) => {
                        let missing: Vec<Command<_>> = miss
                            .iter()
                            .map(|x| {
                                Command::perform(
                                    playlist
                                        .source
                                        .get_video(self.web.get_browser(), x.get_url()),
                                    Message::ResultVideo,
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
            Message::UpdatePlaylist(Ok(playlist)) => {
                let comm = match self.data.get_missing_videos(&playlist.videos) {
                    Some(miss) => {
                        let missing: Vec<Command<_>> = miss
                            .iter()
                            .map(|x| {
                                Command::perform(
                                    playlist
                                        .source
                                        .get_video(self.web.get_browser(), x.get_url()),
                                    Message::UpdateVideo,
                                )
                            })
                            .collect();
                        Command::batch(missing)
                    }
                    None => Command::none(),
                };
                if let Err(e) = self.data.update_playlist(playlist) {
                    self.status.report(format!("{e}"));
                    return Command::none();
                } else {
                    return comm;
                }
            }
            Message::ResultPlaylist(Err(err)) => {
                println!("{err}");
                self.status.report(format!("{err}"));
                Command::none()
            }
            Message::UpdatePlaylist(Err(e)) => {
                println!("{e}");
                self.status.report(format!("{e}"));
                Command::none()
            }
            Message::ResultVideo(Ok(video)) => {
                if let Err(e) = self.data.add_video(video) {
                    self.status.report(format!("Failed to add video: {}", e));
                }
                Command::none()
            }
            Message::UpdateVideo(Ok(v)) => {
                if let Err(e) = self.data.update_video(v) {
                    self.status.report(format!("{e}"));
                }
                Command::none()
            }
            Message::UpdateVideo(Err(e)) => {
                println!("{e}");
                self.status.report(format!("{e}"));
                Command::none()
            }
            Message::ResultVideo(Err(e)) => {
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
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let bar = self.side_bar_view();

        let main = match &self.screen {
            WindowScreen::PlaylistTracker => self.playlist_tracker_view(),
            WindowScreen::PlaylistDetail(id) => self.playlist_detail_view(id),
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
    fn playlist_tracker_view(&self) -> Element<Message> {
        let bar = row!(
            text_input(
                "Add Playlist URL",
                &self.inputs.add_playlist,
                Message::AddPlaylistURL
            ),
            button("Add").on_press(Message::AddPlaylist)
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
