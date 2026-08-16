#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn scale(&self, k: f64) -> Self {
        Vec3 {
            x: k * self.x,
            y: k * self.y,
            z: k * self.z,
        }
    }

    pub fn add(&self, other: &Vec3) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn dist_to(&self, p: &Point3) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        let dz = self.z - p.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

}

#[cfg(test)]
mod tests {
    use crate::geometry::test_utils::assert_approx_eq;

use super::*;

    #[test]
    fn vec3_dist_to() {
        let v = Vec3::new(0.0, 0.0, 0.0);
        let p = Point3::new(1.0, 1.0, 1.0);
        let dist = v.dist_to(&p);
        assert_approx_eq(dist, (3f64).sqrt());
    }
}
