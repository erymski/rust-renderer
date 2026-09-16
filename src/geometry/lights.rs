use crate::geometry::{Color, Vec3};

/// it's like a sun
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct AmbientLight {
    pub color: Color,
}

impl AmbientLight {
    pub const fn with_intensity(color: &Color, intensity: f64) -> Self {
        Self {
            color: color.scale(intensity),
        }
    }
}
