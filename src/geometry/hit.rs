use crate::geometry::{Point3, Vec3};

#[derive(Debug)]
pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material_index: usize,
}

impl Hit {
    pub fn new(point: Point3, normal: Vec3, t: f64, material_index: usize) -> Self {
        Hit {
            point,
            normal,
            t,
            material_index,
        }
    }
}
