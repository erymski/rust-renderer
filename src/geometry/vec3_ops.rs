use std::ops::{Add, AddAssign};

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
