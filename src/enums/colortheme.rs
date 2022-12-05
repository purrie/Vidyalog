use iced::Theme;

use super::ColorTheme;

impl From<ColorTheme> for Theme {
    fn from(t: ColorTheme) -> Self {
        match t {
            ColorTheme::Light => Theme::Light,
            ColorTheme::Dark => Theme::Dark,
        }
    }
}
