use crate::geometry::Color;

pub type ToneMapper = fn(&Color) -> Color;

/// just return the color itself
#[allow(dead_code)]
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

fn reinhard(f: f64) -> f64 {
    f / (1. + f)
}

/// Reinhard color mapping
#[allow(dead_code)]
pub fn color_reinhard(color: &Color) -> Color {
    Color {
        x: reinhard(color.x),
        y: reinhard(color.y),
        z: reinhard(color.z),
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::test_utils::{assert_approx_eq, assert_vec3_eq};

    use super::*;

    #[test]
    fn clamp_check() {
        let color = Color::new(-5., 12., 0.);
        assert_vec3_eq(&color_clamp(&color), &Color::new(0., 1., 0.));
    }

    #[test]
    fn reinhard_check() {
        assert_approx_eq(reinhard(0.), 0.);
        assert_approx_eq(reinhard(1.), 0.5);
        assert_approx_eq(reinhard(100_000_000_000_000.), 1.0);
    }

    fn reinhard_color_check() {
        let color = Color::new(1., 12., 0.);
        assert_vec3_eq(&color_clamp(&color), &Color::new(0.5, 12. / 13., 0.));
    }
}
