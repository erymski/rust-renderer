use crate::geometry::{Hit, Hittable, Point3, Ray, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.from.sub(&self.center);
        let oc_len = oc.length();

        let b = -2.0 * oc.dot(&ray.dir); // negate immediately, minor optimization to avoid negating later
        let c = (oc_len * oc_len) - (self.radius * self.radius);
        let discriminant = b * b - 4.0 * c;

        if discriminant < 0.0 {
            None
        } else {
            let d_sqrt = discriminant.sqrt();
            let mut t = b - d_sqrt; // closest intersection point
            if t < 0.0 {
                // D == 0 means "single result", kind-of touching sphere
                if discriminant < 1e-10 {
                    return None;
                }

                t = b + d_sqrt; // farthest intersection point
                if t < 0.0 {
                    return None;
                }
            }

            t *= 0.5;

            let point = ray.from.add(&ray.dir.scale(t));
            let normal = point.sub(&self.center).normalize();
            Some(Hit::new(point, normal, t))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::test_utils::assert_vec3_eq;

    use super::*;

    const SPHERE: Sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 10.0);

    // TODO: need more tests for sphere hit/miss, including edge cases and rays that start inside the sphere

    #[test]
    fn sphere_hit() {
        let rays = [
            Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
            Ray::new(Vec3::new(0.0, 0.0, 9.0), Vec3::new(0.0, 0.0, 1.0)),
            Ray::new(Vec3::new(0.0, 0.0, 11.0), Vec3::new(0.0, 0.0, -1.0)),
        ];

        for ray in rays {
            let hit = SPHERE.hit(&ray);
            assert!(hit.is_some());
            assert_vec3_eq(&hit.unwrap().point, &Vec3::new(0.0, 0.0, 10.0));
        }
    }

    #[test]
    fn sphere_miss() {
        let rays = [
            Ray::new(Point3::new(0.0, 0.0, -15.0), Vec3::new(0.0, 0.0, -1.0)),
            Ray::new(Point3::new(0.0, 0.0, -15.0), Vec3::new(0.0, 1.0, 0.0)),
            Ray::new(Point3::new(0.0, 0.0, 15.0), Vec3::new(0.0, 0.0, 1.0)),
        ];
        for ray in rays {
            let hit = SPHERE.hit(&ray);
            assert!(hit.is_none());
        }
    }
}
