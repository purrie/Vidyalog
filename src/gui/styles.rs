use super::{StyleAdjustment, Styles};
use iced::{theme, widget, Background, Theme};

impl widget::container::StyleSheet for Styles {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> widget::container::Appearance {
        use widget::container::Appearance;

        let palette = style.palette();
        match self {
            Styles::Box => {
                let bgc = palette.background.darker();
                Appearance {
                    background: Some(Background::Color(bgc)),
                    border_width: 2.,
                    ..Default::default()
                }
            }
            Styles::Header => {
                let bgc = palette.background.darker().darker();
                Appearance {
                    background: Some(Background::Color(bgc)),
                    border_width: 0.,
                    border_radius: 0.,
                    ..Default::default()
                }
            }
            Styles::Danger => {
                let bgc = palette.background.redden();
                Appearance {
                    background: Some(Background::Color(bgc)),
                    ..Default::default()
                }
            }
        }
    }
}
impl widget::button::StyleSheet for Styles {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> widget::button::Appearance {
        use widget::button::Appearance;

        let palette = style.palette();
        match self {
            Styles::Box => todo!(),
            Styles::Header => todo!(),
            Styles::Danger => {
                let col = palette.danger;
                Appearance {
                    background: Some(Background::Color(col)),
                    ..Default::default()
                }
            }
        }
    }
    fn hovered(&self, style: &Self::Style) -> widget::button::Appearance {
        use widget::button::Appearance;

        let palette = style.palette();
        match self {
            Styles::Box => todo!(),
            Styles::Header => todo!(),
            Styles::Danger => {
                let col = palette.danger.lighter();
                Appearance {
                    background: Some(Background::Color(col)),
                    ..Default::default()
                }
            }
        }
    }
}
impl From<Styles> for theme::Container {
    fn from(s: Styles) -> Self {
        theme::Container::Custom(Box::new(s))
    }
}
impl From<Styles> for theme::Button {
    fn from(s: Styles) -> Self {
        theme::Button::Custom(Box::new(s))
    }
}
