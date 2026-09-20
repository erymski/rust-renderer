use std::ops::{Add, AddAssign, Sub, SubAssign};

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
