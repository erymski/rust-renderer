pub struct Camera {
    pub eye: Point3,
    pub up: Vec3,
    /// direction and distance to viewport (length is important)
    pub vpDir: Vec3,
}

pub struct Viewport {
    width: i32,
    height: i32,
}
