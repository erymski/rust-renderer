use crate::geometry::Color;

pub type ToneMapper = fn(&Color) -> Color;

/// just return the color itself
pub fn color_identity(color: &Color) -> Color {
    color.clone()
}
