use iced::Color;

use super::StyleAdjustment;

impl StyleAdjustment for Color {
    fn lighter(self) -> Self {
        let Color { a, r, g, b } = self;
        let r = r + 0.03;
        let g = g + 0.03;
        let b = b + 0.03;
        Self { a, r, g, b }
    }

    fn darker(self) -> Self {
        let Color { a, r, g, b } = self;
        let r = r - 0.03;
        let g = g - 0.03;
        let b = b - 0.03;
        Self { a, r, g, b }
    }

    fn redder(self) -> Self {
        let Color { a, r, g, b } = self;
        let g = g * 0.8;
        let b = b * 0.8;
        Self { a, r, g, b }
    }
}
