use crate::geometry::Color;

pub type ToneMapper = fn(&Color) -> Color;

/// just return the color itself
pub fn color_identity(color: &Color) -> Color {
    color.clone()
}

#[allow(dead_code)]
pub fn color_clamp(color: &Color) -> Color {
    Color {
        x: color.x.max(0.).min(1.0),
        y: color.y.max(0.).min(1.0),
        z: color.z.max(0.).min(1.0),
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::test_utils::assert_vec3_eq;

    use super::*;

    #[test]
    fn clamp_check() {
        let color = Color::new(-5., 12., 0.);
        assert_vec3_eq(&color_clamp(&color), &Color::new(0., 1., 0.));
    }
}
