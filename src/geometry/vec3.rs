#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_to(from: &Point3, to: &Point3) -> Self {
        to.sub(&from)
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self.scale(1.0 / self.length())
    }

    pub fn scale(&self, k: f64) -> Self {
        Self::new(k * self.x, k * self.y, k * self.z)
    }

    pub fn add(&self, other: &Vec3) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Vec3) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn dist_to(&self, p: &Point3) -> f64 {
        self.sub(p).length()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::test_utils::{assert_approx_eq, assert_vec3_eq};

    use super::*;

    #[test]
    fn vec3_from_to() {
        let from = Vec3::new(-1.0, -2.0, 0.0);
        let to = Point3::new(1.0, 3.0, 1.0);
        assert_vec3_eq(&Vec3::from_to(&from, &to), &Vec3::new(2.0, 5., 1.));
    }

    #[test]
    fn vec3_dist_to() {
        let v = Vec3::new(0.0, 0.0, 0.0);
        let p = Point3::new(1.0, 1.0, 1.0);
        let dist = v.dist_to(&p);
        assert_approx_eq(dist, (3f64).sqrt());
    }

    #[test]
    fn vec3_scale() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let scaled = v.scale(2.0);
        assert_vec3_eq(&scaled, &Vec3::new(2.0, 4., 6.));
    }

    #[test]
    fn vec3_length() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        let len = v.length();
        assert_approx_eq(len, 3.0);
    }

    #[test]
    fn vec3_normalize() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        let normalized = v.normalize();
        assert_approx_eq(normalized.length(), 1.0);
    }

    #[test]
    fn vec3_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, -5.0, 6.0);
        let dot = v1.dot(&v2);
        assert_approx_eq(dot, 12.0);
    }

    #[test]
    fn vec3_dot_opposite() {
        let v1 = Vec3::new(1.0, 2.0, 3.0).normalize();
        let v2 = Vec3::new(-1.0, -2.0, -3.0).normalize();
        let dot = v1.dot(&v2);
        assert_approx_eq(dot, -1.0);
    }

    #[test]
    fn vec3_dot_perpendicular() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let dot = v1.dot(&v2);
        assert_approx_eq(dot, 0.0);
    }
}
