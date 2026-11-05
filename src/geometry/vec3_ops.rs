use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign};

use crate::geometry::Vec3;

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Vec3::add(&self, &rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        Vec3::add_mut(self, &rhs);
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::sub(&self, &rhs)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        Vec3::sub_mut(self, &rhs);
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::mult(&self, &rhs)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        Vec3::mult_mut(self, &rhs);
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::scale(&self, rhs)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        Vec3::scale_mut(self, rhs);
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::div(&self, rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        Vec3::div_mut(self, rhs);
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

mod tests {
    use super::*;
    use crate::geometry::V;

    #[test]
    fn vector_indexes() {
        let v = V(1.0, 2.0, 3.0);
        assert_eq!(v[0], v.x);
        assert_eq!(v[1], v.y);
        assert_eq!(v[2], v.z);
    }

    #[test]
    #[should_panic]
    fn vector_bad_index() {
        let vec = V(1.0, 2.0, 3.0);
        let _ = vec[3];
    }
}
