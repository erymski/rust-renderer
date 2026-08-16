struct Ray {
    from: Vec3,
    dir: Vec3, // direction vector, normalized
    len: f64,
}

impl Ray {
    fn new(from: Vec3, dir: Vec3, len: f64) -> Self {
        Ray { from, dir, len }
    }
}
