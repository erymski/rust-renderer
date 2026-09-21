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
        Ray::new(from, *to - from)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::geometry::test_utils::assert_approx_eq;
    use crate::geometry::vec3::{P, V};

    #[test]
    fn ray_from_two_points() {
        let from = P(0.0, 0.0, 0.0);
        let to = P(1.0, 1.0, 1.0);
        let ray = Ray::from_points(from, &to);

        assert_approx_eq(ray.from.x, 0.0);
        assert_approx_eq(ray.from.y, 0.0);
        assert_approx_eq(ray.from.z, 0.0);

        let normalized: f64 = 1.0 / (3f64).sqrt();

        assert_approx_eq(ray.dir.x, normalized);
        assert_approx_eq(ray.dir.y, normalized);
        assert_approx_eq(ray.dir.z, normalized);
    }

    #[test]
    fn ray_should_be_normalized() {
        let ray = Ray::new(P(1., 2., 3.), V(5., 78., 32.));
        assert_approx_eq(ray.dir.length(), 1.);
    }
}
