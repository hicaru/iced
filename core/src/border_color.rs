use crate::Color;

/// The border colors for the corners of a graphics primitive in the order:
/// top, right, bottom, left.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderColor([Color; 4]);

impl Default for BorderColor {
    fn default() -> Self {
        BorderColor::from(Color::TRANSPARENT)
    }
}

impl From<Color> for BorderColor {
    fn from(color: Color) -> Self {
        Self([color; 4])
    }
}

impl From<[Color; 4]> for BorderColor {
    fn from(colors: [Color; 4]) -> Self {
        Self(colors)
    }
}

impl From<BorderColor> for [Color; 4] {
    fn from(bc: BorderColor) -> Self {
        bc.0
    }
}

impl BorderColor {
    /// get top broder color
    pub fn top(&self) -> Color {
        self.0[0]
    }
    /// get bottom broder color
    pub fn bottom(&self) -> Color {
        self.0[2]
    }
    /// get left broder color
    pub fn left(&self) -> Color {
        self.0[3]
    }
    /// get right broder color
    pub fn right(&self) -> Color {
        self.0[1]
    }
}
