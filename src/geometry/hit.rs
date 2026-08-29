use crate::geometry::{Color, Point3, Vec3};

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub color: Color,
}

impl Hit {
    pub fn new(point: Point3, normal: Vec3, t: f64, color: Color) -> Self {
        Hit {
            point,
            normal,
            t,
            color,
        }
    }
}
