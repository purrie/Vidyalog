use iced::{
    widget::{button, column, container, row},
    Application, Command, Length,
};

use crate::{data::Playlist, enums::Message, file::File};

use super::Window;

impl Application for Window {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let playlists = Playlist::load_all();
        let mut app = Window::default();
        app.playlist_tracker.playlists = playlists;
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Vidyalog".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::AddPlaylistURL(url) => {
                self.playlist_tracker.add_url = url;
                Command::none()
            }
            Message::Test => {
                let test =
                    "https://www.youtube.com/playlist?list=PLOuKvPOY7ipL4Et2aaChM1akMUYjdKnBJ"
                        .to_string();
                Command::perform(self.web.get_playlist(test), Message::ResultPlaylist)
            }
            Message::AddPlaylist => {
                let p = std::mem::take(&mut self.playlist_tracker.add_url);
                Command::perform(self.web.get_playlist(p), Message::ResultPlaylist)
            }
            Message::ResultPlaylist(Ok(playlist)) => {
                match playlist.save() {
                    Ok(_) => {
                        self.status
                            .report(format!("Successfully loaded {}", &playlist.title));
                        self.playlist_tracker.add_playlist(playlist);
                    }
                    Err(e) => self
                        .status
                        .report(format!("Failed loading the playlist due to {}", e)),
                }
                Command::none()
            }
            Message::ResultPlaylist(Err(err)) => {
                println!("{err}");
                self.status.report(format!("{err}"));
                Command::none()
            }
            Message::DeletePlaylist(id) => {
                if let Some(x) = self.playlist_tracker.remove(&id) {
                    if let Err(e) = x.delete() {
                        self.status
                            .report(format!("Failed to remove playlist: {}", e));
                    } else {
                        self.status.report(format!("Removed {}", id));
                    }
                }
                Command::none()
            }
            Message::OpenInBrowser(url) => {
                open::that(url).unwrap();
                Command::none()
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let bar = column![button("Test").on_press(Message::Test),].width(Length::Shrink);

        let main = match self.screen {
            super::WindowScreen::PlaylistMod => self.playlist_tracker.view(),
        };

        let content = column![row![bar, main].height(Length::Fill), self.status.line()];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
