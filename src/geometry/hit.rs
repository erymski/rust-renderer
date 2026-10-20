use crate::geometry::{Point3, Vec3, make_offset};

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material_index: usize,
}

impl Hit {
    pub fn new(point: Point3, normal: Vec3, t: f64, material_index: usize) -> Self {
        Hit {
            point,
            normal,
            t,
            material_index,
        }
    }

    /// make a slightly offset point, which moves from hit point in normal direction
    pub fn calc_offset(self) -> Point3 {
        make_offset(self.point, self.normal)
    }
}

#[cfg(test)]
mod test {

    use crate::geometry::{P, V, test_utils::assert_approx_eq};

    use super::*;

    #[test]
    fn offset_is_in_normal_direction() {
        let hit = Hit::new(P(1., 2., -5.), V(0.36, -88.33, 1.55), 3., 0);
        let offset = hit.calc_offset();

        let dir = (offset - hit.point).normalize();
        assert_approx_eq(dir.dot(&hit.normal.normalize()), 1.0);
    }
}
