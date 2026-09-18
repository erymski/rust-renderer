use crate::geometry::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    pub from: Point3,
    /// Normalized direction vector
    pub dir: Vec3,
}

impl Ray {
    pub fn new(from: Point3, dir: Vec3) -> Self {
        Ray {
            from,
            dir: dir.normalize(),
        }
    }

    pub fn from_points(from: Point3, to: &Point3) -> Self {
        let delta = to.sub(&from);
        Ray::new(from, delta)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::geometry::test_utils::assert_approx_eq;

    #[test]
    fn ray_from_two_points() {
        let from = Point3::new(0.0, 0.0, 0.0);
        let to = Point3::new(1.0, 1.0, 1.0);
        let ray = Ray::from_points(from, &to);

        assert_approx_eq(ray.from.x, 0.0);
        assert_approx_eq(ray.from.y, 0.0);
        assert_approx_eq(ray.from.z, 0.0);

        let normalized: f64 = 1.0 / (3f64).sqrt();

        assert_approx_eq(ray.dir.x, normalized);
        assert_approx_eq(ray.dir.y, normalized);
        assert_approx_eq(ray.dir.z, normalized);
    }
}
