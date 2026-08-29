use crate::geometry::{Point3, Vec3};

pub struct Camera {
    pub eye: Point3,
    pub up: Vec3,
    /// direction and distance to viewport (length is important)
    pub vp_dir: Vec3,
}

pub struct Viewport {
    pub width: u32,
    pub height: u32,
}
