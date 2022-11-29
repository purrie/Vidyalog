use super::Styles;
use iced::{theme, widget, Background, Theme};

impl widget::container::StyleSheet for Styles {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> widget::container::Appearance {
        use widget::container::Appearance;

        let ext = style.extended_palette();
        match self {
            Styles::ContentFrame => {
                let bgc = ext.background.weak.color;
                let text = ext.background.weak.text;
                Appearance {
                    background: Some(Background::Color(bgc)),
                    text_color: Some(text),
                    ..Default::default()
                }
            }
            Styles::Header => {
                let bgc = ext.background.base.color;
                let text = ext.background.base.text;
                Appearance {
                    background: Some(Background::Color(bgc)),
                    text_color: Some(text),
                    border_width: 0.,
                    border_radius: 0.,
                    ..Default::default()
                }
            }
            Styles::Danger => todo!(),
            Styles::Background => {
                let back = ext.background.strong.color;
                let text = ext.background.strong.text;
                Appearance {
                    background: Some(Background::Color(back)),
                    text_color: Some(text),
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

        let ext = style.extended_palette();
        match self {
            Styles::ContentFrame => {
                let col = ext.secondary.weak.color;
                let text = ext.secondary.weak.text;

                Appearance {
                    background: Some(Background::Color(col)),
                    text_color: text,
                    ..Default::default()
                }
            }
            Styles::Header => todo!(),
            Styles::Danger => {
                let col = ext.danger.base.color;
                let text = ext.danger.base.text;
                Appearance {
                    background: Some(Background::Color(col)),
                    text_color: text,
                    ..Default::default()
                }
            }
            Styles::Background => todo!(),
        }
    }
    fn hovered(&self, style: &Self::Style) -> widget::button::Appearance {
        use widget::button::Appearance;

        let ext = style.extended_palette();
        match self {
            Styles::ContentFrame => {
                let col = ext.secondary.strong.color;
                let text = ext.secondary.strong.text;

                Appearance {
                    background: Some(Background::Color(col)),
                    text_color: text,
                    ..Default::default()
                }
            }
            Styles::Header => todo!(),
            Styles::Danger => {
                let col = ext.danger.strong.color;
                let text = ext.danger.strong.text;
                Appearance {
                    background: Some(Background::Color(col)),
                    text_color: text,
                    ..Default::default()
                }
            }
            Styles::Background => todo!(),
        }
    }
    fn pressed(&self, style: &Self::Style) -> widget::button::Appearance {
        use widget::button::Appearance;

        let ext = style.extended_palette();
        match self {
            Styles::Background => todo!(),
            Styles::ContentFrame => {
                let col = ext.secondary.weak.color;
                let text = ext.secondary.weak.text;

                Appearance {
                    background: Some(Background::Color(col)),
                    text_color: text,
                    ..Default::default()
                }
            }
            Styles::Header => todo!(),
            Styles::Danger => {
                let col = ext.danger.base.color;
                let text = ext.danger.base.text;
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

        let ext = style.extended_palette();
        match self {
            Styles::ContentFrame => {
                let col = ext.background.strong.color;

                Appearance {
                    width: 2,
                    radius: 2.,
                    fill_mode: widget::rule::FillMode::Full,
                    color: col,
                }
            }
            Styles::Header => {
                let col = ext.background.base.color;

                Appearance {
                    width: 2,
                    radius: 2.,
                    fill_mode: widget::rule::FillMode::Full,
                    color: col,
                }
            }
            Styles::Danger => todo!(),
            Styles::Background => todo!(),
        }
    }
}
impl widget::scrollable::StyleSheet for Styles {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> widget::scrollable::Scrollbar {
        use widget::scrollable::Scrollbar;
        use widget::scrollable::Scroller;

        let ext = style.extended_palette();
        match self {
            Styles::Background => todo!(),
            Styles::ContentFrame => {
                let col = ext.secondary.base.color;
                let border = ext.secondary.weak.color;

                let scroller = ext.primary.weak.color;
                let sc_bor = ext.primary.weak.color;

                Scrollbar {
                    background: Some(Background::Color(col)),
                    border_radius: 4.,
                    border_width: 4.,
                    border_color: border,
                    scroller: Scroller {
                        color: scroller,
                        border_radius: 1.,
                        border_width: 1.,
                        border_color: sc_bor,
                    },
                }
            }
            Styles::Header => todo!(),
            Styles::Danger => todo!(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> widget::scrollable::Scrollbar {
        use widget::scrollable::Scrollbar;
        use widget::scrollable::Scroller;

        let ext = style.extended_palette();
        match self {
            Styles::Background => todo!(),
            Styles::ContentFrame => {
                let col = ext.secondary.base.color;
                let border = ext.secondary.weak.color;

                let scroller = ext.primary.strong.color;
                let sc_bor = ext.primary.weak.color;

                Scrollbar {
                    background: Some(Background::Color(col)),
                    border_radius: 4.,
                    border_width: 4.,
                    border_color: border,
                    scroller: Scroller {
                        color: scroller,
                        border_radius: 3.,
                        border_width: 3.,
                        border_color: sc_bor,
                    },
                }
            }
            Styles::Header => todo!(),
            Styles::Danger => todo!(),
        }
    }
    fn dragging(&self, style: &Self::Style) -> widget::scrollable::Scrollbar {
        use widget::scrollable::Scrollbar;
        use widget::scrollable::Scroller;

        let ext = style.extended_palette();
        match self {
            Styles::Background => todo!(),
            Styles::ContentFrame => {
                let col = ext.secondary.base.color;
                let border = ext.secondary.weak.color;

                let scroller = ext.primary.base.color;
                let sc_bor = ext.primary.strong.color;

                Scrollbar {
                    background: Some(Background::Color(col)),
                    border_radius: 4.,
                    border_width: 4.,
                    border_color: border,
                    scroller: Scroller {
                        color: scroller,
                        border_radius: 4.,
                        border_width: 4.,
                        border_color: sc_bor,
                    },
                }
            }
            Styles::Header => todo!(),
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
impl From<Styles> for theme::Scrollable {
    fn from(s: Styles) -> Self {
        theme::Scrollable::Custom(Box::new(s))
    }
}
