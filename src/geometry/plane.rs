use crate::geometry::{Color, Hit, Hittable, Point3, Ray, Vec3, tolerance::is_zero, vec3_eq};
pub struct Plane {
    /// point of the plane
    pub point: Point3,

    /// normal to the plane. Expected to be a unit vector.
    pub normal: Vec3,

    pub color: Color,
}

impl Plane {
    /// create new plane with unitized normal
    pub fn new(point: Point3, normal: &Vec3, color: Color) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            color,
        }
    }
}

impl Hittable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        if vec3_eq(&ray.from, &self.point) {
            return Some(Hit::new(self.point, self.normal, 0.0, self.color));
        }

        let dn = ray.dir.dot(&self.normal);
        if is_zero(dn) {
            // ray perpendicular to the plane normal
            return None; // TODO: right?
        }

        let k = self.point.sub(&ray.from);
        let kn = k.dot(&self.normal);
        if is_zero(kn) {
            // ray is parallel to the plane
            return None;
        }

        let t = kn / dn;
        let intersection = ray.from.add(&ray.dir.scale(t));

        Some(Hit::new(intersection, self.normal, t, self.color))
    }
}

#[cfg(test)]
mod tests {

    use crate::geometry::colors;
    use crate::geometry::test_utils::{assert_approx_eq, assert_vec3_eq};

    use super::*;

    #[test]
    fn plane_intersection_simple_orientation() {
        let plane = Plane::new(
            Point3::new(0., 0., 0.),
            &Vec3::new(1., 0., 0.),
            colors::BLUE,
        );

        let ray = Ray::from_points(Point3::new(10., 2., 6.), &Point3::new(-3., 2., 6.));
        let intersection = plane.intersect(&ray).unwrap();
        assert_vec3_eq(&intersection.point, &Point3::new(0., 2., 6.));
        assert_vec3_eq(&intersection.normal, &plane.normal);
        assert_vec3_eq(&intersection.color, &colors::BLUE);
        assert_approx_eq(intersection.t, 10.);
    }

    #[test]
    fn plane_intersection_complex_orientation() {
        let plane = Plane::new(
            Point3::new(1., 2., 3.),
            &Vec3::new(2., -1., 1.),
            colors::GREEN,
        );

        let ray = Ray::from_points(Point3::new(0., 0., 0.), &Point3::new(2., 1., 1.));
        let intersection = plane.intersect(&ray).unwrap();
        assert_vec3_eq(&intersection.point, &Point3::new(1.5, 0.75, 0.75));
        assert_vec3_eq(&intersection.normal, &plane.normal);
        assert_vec3_eq(&intersection.color, &colors::GREEN);
        assert_approx_eq(intersection.t, 1.837_117_307_08);
    }

    // TODO: more tests for intersection, including None cases
}
