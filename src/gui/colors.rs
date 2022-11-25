use iced::Color;

use super::StyleAdjustment;

impl StyleAdjustment for Color {
    fn lighter(self) -> Self {
        let Color { a, r, g, b } = self;
        let r = r + 0.1;
        let g = g + 0.1;
        let b = b + 0.1;
        Self { a, r, g, b }
    }

    fn darker(self) -> Self {
        let Color { a, r, g, b } = self;
        let r = r - 0.1;
        let g = g - 0.1;
        let b = b - 0.1;
        Self { a, r, g, b }
    }

    fn redden(self) -> Self {
        let Color { a, r, g, b } = self;
        let g = g * 0.8;
        let b = b * 0.8;
        Self { a, r, g, b }
    }
}
