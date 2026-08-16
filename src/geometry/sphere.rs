use crate::geometry::{Hit3d, Hittable3d, Point3, Ray};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, r: f64) -> Self {
        Sphere { center, radius: r }
    }
}

impl Hittable3d for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit3d> {
        let oc = ray.from.sub(&self.center);

        let a = ray.dir.length().powi(2);
        let b = 2.0 * oc.dot(&ray.dir);
        let c = oc.length().powi(2) - self.radius.powi(2);
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t < 0.0 || t > ray.len {
                return None;
            }
            let point = ray.from.add(&ray.dir.scale(t));
            let normal = point.sub(&self.center).normalize();
            Some(Hit3d { point, normal })
        }
    }
}
