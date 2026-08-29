use crate::geometry::{Point3, Vec3};

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl Hit {
    pub fn new(point: Point3, normal: Vec3, t: f64) -> Self {
        Hit { point, normal, t }
    }
}
