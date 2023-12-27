use crate::Color;

/// The border radii for the corners of a graphics primitive in the order:
/// top-left, top-right, bottom-right, bottom-left.
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
