use super::{StyleAdjustment, Styles};
use iced::{theme, widget, Background};

impl From<Styles> for theme::Container {
    fn from(s: Styles) -> Self {
        theme::Container::Custom(Box::new(s))
    }
}
impl widget::container::StyleSheet for Styles {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> widget::container::Appearance {
        let palette = style.palette();
        match self {
            Styles::Box => {
                let bgc = palette.background.darker();
                widget::container::Appearance {
                    background: Some(Background::Color(bgc)),
                    border_width: 2.,
                    ..Default::default()
                }
            }
            Styles::Header => {
                let bgc = palette.background.darker().darker();
                widget::container::Appearance {
                    background: Some(Background::Color(bgc)),
                    border_width: 2.,
                    border_radius: 2.,
                    ..Default::default()
                }
            }
        }
    }
}
