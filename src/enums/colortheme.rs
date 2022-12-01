use iced::Theme;

use super::ColorTheme;

impl From<Theme> for ColorTheme {
    fn from(t: Theme) -> Self {
        match t {
            Theme::Light => Self::Light,
            Theme::Dark => Self::Dark,
            Theme::Custom(_) => todo!(),
        }
    }
}
impl From<ColorTheme> for Theme {
    fn from(t: ColorTheme) -> Self {
        match t {
            ColorTheme::Light => Theme::Light,
            ColorTheme::Dark => Theme::Dark,
        }
    }
}
