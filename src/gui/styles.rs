use super::{StyleAdjustment, Styles};
use iced::{theme, widget, Background, Theme};

impl widget::container::StyleSheet for Styles {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> widget::container::Appearance {
        use widget::container::Appearance;

        let palette = style.palette();
        match self {
            Styles::Distinguished => {
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
                let bgc = palette.background.redder();
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
            Styles::Distinguished => todo!(),
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
            Styles::Distinguished => todo!(),
            Styles::Header => todo!(),
            Styles::Danger => {
                let col = palette.danger.lighter();
                let text = palette.text.darker().darker().darker();
                Appearance {
                    background: Some(Background::Color(col)),
                    text_color: text,
                    ..Default::default()
                }
            }
        }
    }
}
impl widget::rule::StyleSheet for Styles {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> widget::rule::Appearance {
        use widget::rule::Appearance;

        let palette = style.palette();
        let light = match style {
            Theme::Light => true,
            Theme::Dark => false,
            Theme::Custom(_) => {
                let bg = palette.background;
                let bg = bg.r + bg.b + bg.g;
                if bg > 0.5 {
                    true
                } else {
                    false
                }
            }
        };
        match self {
            Styles::Distinguished => {
                let col = if light {
                    palette.background.darker().darker()
                } else {
                    palette.background.lighter().lighter()
                };

                Appearance {
                    width: 2,
                    radius: 2.,
                    fill_mode: widget::rule::FillMode::Full,
                    color: col,
                }
            }
            Styles::Header => {
                let col = if light {
                    palette.background.darker().darker().darker()
                } else {
                    palette.background.lighter().lighter().lighter()
                };

                Appearance {
                    width: 2,
                    radius: 2.,
                    fill_mode: widget::rule::FillMode::Full,
                    color: col,
                }
            }
            Styles::Danger => todo!(),
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
impl From<Styles> for theme::Rule {
    fn from(s: Styles) -> Self {
        theme::Rule::Custom(Box::new(s))
    }
}
