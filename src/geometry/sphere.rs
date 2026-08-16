use crate::geometry::{Hit3d, Hittable3d, Point3, Ray, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable3d for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit3d> {
        let oc = ray.from.sub(&self.center);

        let a = ray.dir.length() * ray.dir.length();
        let b = 2.0 * oc.dot(&ray.dir);
        let c = oc.length().powi(2) - (self.radius * self.radius);
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let d_sqrt = discriminant.sqrt();
            let mut t = (-b - d_sqrt) / (2.0 * a);
            if t < 0.0 {
                t = (-b + d_sqrt) / (2.0 * a);
                if t < 0.0 {
                    return None;
                }
            }
            let point = ray.from.add(&ray.dir.scale(t));
            let normal = point.sub(&self.center).normalize();
            Some(Hit3d::new(point, normal, t))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPHERE: Sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 10.0);

    // TODO: need more tests for sphere hit/miss, including edge cases and rays that start inside the sphere

    #[test]
    fn sphere_hit() {
        const RAY: Ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = SPHERE.hit(&RAY);
        assert!(hit.is_some());
    }

    #[test]
    fn sphere_miss() {
        const RAY: Ray = Ray::new(Point3::new(0.0, 0.0, -15.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = SPHERE.hit(&RAY);
        assert!(hit.is_none());
    }
}
