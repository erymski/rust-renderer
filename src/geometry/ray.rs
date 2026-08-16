use crate::geometry::vec3::{Point3, Vec3};

pub struct Ray {
    pub from: Point3,
    /// Normalized direction vector
    pub dir: Vec3,
    pub len: f64, // TODO: do we need it?
}

impl Ray {
    pub fn new(from: Point3, dir: Vec3, len: f64) -> Self {
        Ray { from, dir, len }
    }

    pub fn from_points(from: Point3, to: Point3) -> Self {
        let delta = to.sub(&from);
        let dir = delta.normalize();
        let len = delta.length();
        Ray { from, dir, len }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ray_from_two_points() {
        let from = Point3::new(0.0, 0.0, 0.0);
        let to = Point3::new(1.0, 1.0, 1.0);
        let ray = Ray::from_points(from, to);

        assert_eq!(ray.from.x, 0.0);
        assert_eq!(ray.from.y, 0.0);
        assert_eq!(ray.from.z, 0.0);

        let normalized: f64 = 1.0 / (3f64).sqrt();

        assert_eq!(ray.dir.x, normalized);
        assert_eq!(ray.dir.y, normalized);
        assert_eq!(ray.dir.z, normalized);

        assert_eq!(ray.len, (3f64).sqrt());
    }
}
