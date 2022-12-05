use iced::widget::{column, container, horizontal_space, radio, row, text};
use iced::{Alignment, Length};

use crate::enums::{ColorTheme, Message};
use crate::file::{File, SettingsPath, SingleFileID};
use crate::gui::{ListView, Styles};
use crate::program::Settings;

impl ListView for Settings {
    /// This is the main UI view for the program settings
    fn gui_list_view<'a>(
        &self,
        _database: &crate::program::Database,
    ) -> iced::Element<'a, crate::enums::Message> {
        let theme = row![
            horizontal_space(Length::Fill),
            text("Theme"),
            horizontal_space(Length::Fill),
            column![
                radio(
                    "Light",
                    ColorTheme::Light,
                    Some(self.theme),
                    Message::SetTheme
                ),
                radio(
                    "Dark",
                    ColorTheme::Dark,
                    Some(self.theme),
                    Message::SetTheme
                ),
            ]
            .spacing(4),
            horizontal_space(Length::Fill),
        ]
        .padding(4)
        .spacing(4)
        .align_items(Alignment::Center);
        let theme = container(theme)
            .style(Styles::ContentFrame)
            .height(Length::Shrink)
            .width(Length::Fill);

        let content = row![theme, horizontal_space(Length::Units(4))];
        let content = container(content)
            .style(Styles::Background)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(8);
        content.into()
    }
}
impl File for Settings {
    type Path = SettingsPath;
}
impl Default for Settings {
    /// By default, Settings struct will load the settings file if it exists, otherwise it will return the default values for its settings
    fn default() -> Self {
        if let Ok(s) = Self::load_file() {
            return s;
        }
        Self {
            theme: Default::default(),
        }
    }
}
impl SingleFileID for Settings {
    /// Name of the file the settings are stored
    fn get_file_id() -> &'static str {
        "config"
    }
}
