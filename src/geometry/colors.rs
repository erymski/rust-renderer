use crate::geometry::Color;

pub(crate) const WHITE: Color = Color {
    x: 255.,
    y: 255.,
    z: 255.,
};

pub(crate) const BLUE: Color = Color {
    x: 128.,
    y: 192.,
    z: 255.,
};

/// Linearly interpolates from `a` to `b` by `ratio`.
pub(crate) fn lerp(ratio: f64, a: &Color, b: &Color) -> Color {
    a.scale(1.0 - ratio).add(&b.scale(ratio))
}
