use crate::geometry::{Color, Vec3};

/// it's like a sun
pub struct DirectionalLight {
    pub color: Color,
    pub direction: Vec3,
}

impl DirectionalLight {
    pub const fn with_intensity(color: &Color, direction: Vec3, intensity: f64) -> Self {
        Self {
            color: color.scale(intensity),
            direction,
        }
    }
}

pub struct AmbientLight {
    pub color: Color,
    pub intensity: f32,
}
