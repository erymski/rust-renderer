use crate::geometry::Color;

pub(crate) const WHITE: Color = Color {
    x: 1.,
    y: 1.,
    z: 1.,
};

pub(crate) const BLUE: Color = Color {
    x: 0.5,
    y: 0.75,
    z: 1.,
};

pub(crate) const BLACK: Color = Color {
    x: 0.,
    y: 0.,
    z: 0.,
};

pub(crate) const GREEN: Color = Color {
    x: 0.,
    y: 1.,
    z: 0.,
};

/// Linearly interpolates from `a` to `b` by `ratio`.
pub(crate) fn lerp(ratio: f64, a: &Color, b: &Color) -> Color {
    a.scale(1.0 - ratio) + b.scale(ratio)
}
