use crate::geometry::{Color, Vec3};

/// it's like a sun
pub struct DirectionalLight {
    pub color: Color,
    pub intensity: f32,
    pub direction: Vec3,
}

pub struct AmbientLight {
    pub color: Color,
    pub intensity: f32,
}
