use crate::geometry::{DEFAULT_EPSILON, Hit, Hittable, Point3, Ray, ZERO_VEC, is_zero};

/** Axis aligned box */
#[derive(Debug, Clone, Copy)]
pub struct AABox {
    pub v0: Point3,
    pub v1: Point3,
    pub material_index: usize,
}

impl Hittable for AABox {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut t_min = f64::MIN;
        let mut t_max = f64::MAX;

        let ray_dir = ray.dir();
        let from = ray.from;

        let mut n_min = ZERO_VEC;
        let mut n_max = ZERO_VEC;

        for i in 0..3 {
            let dir = ray_dir[i];
            if is_zero(dir) {
                continue;
            }

            let mut t_in = (self.v0[i] - from[i]) / dir;
            let mut t_out = (self.v1[i] - from[i]) / dir;
            if t_out < t_in {
                std::mem::swap(&mut t_in, &mut t_out);
            }

            let mut n_curr = ZERO_VEC;
            n_curr[i] = if dir < 0.0 { 1.0 } else { -1.0 };

            // looking for *maximum* of minimums
            if t_in > t_min {
                t_min = t_in;
                n_min = n_curr;
            }

            // looking for *minimum* of maximuses
            if t_out < t_max {
                t_max = t_out;
                n_max = n_curr;
            }
        }

        if t_min > t_max {
            return None;
        }

        let (t, n) = if t_min < 0.0 {
            (t_max, n_max)
        } else {
            (t_min, n_min)
        };

        // ray is going away of the box
        if t <= DEFAULT_EPSILON {
            return None;
        }

        let intersection = ray.from + ray_dir * t;

        Some(Hit {
            point: intersection,
            normal: n,
            t: t,
            material_index: self.material_index,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::geometry::{P, V, test_utils::assert_vec3_eq};

    const MATERIAL_INDEX: usize = 3;

    /// check the same box twice, by swapping the vertices
    fn full_box_check(check: fn(AABox)) {
        let mut b = AABox {
            v0: P(0.0, 0.0, 0.0),
            v1: P(5.0, 5.0, 5.0),
            material_index: MATERIAL_INDEX,
        };
        check(b);

        // should work with swapped vertices as well
        std::mem::swap(&mut b.v0, &mut b.v1);
        check(b);
    }

    #[test]
    fn box_diagonal() {
        full_box_check(|b: AABox| {
            let r = Ray::new(P(6.0, 0.0, 1.0), V(-1.0, 1.0, 1.0));
            let res = b.intersect(&r);
            assert!(!res.is_none());

            let hit = res.unwrap();
            assert_vec3_eq(&hit.point, &P(5.0, 1.0, 2.0));
            assert_vec3_eq(&hit.normal, &V(1.0, 0.0, 0.0));
            assert_eq!(hit.material_index, MATERIAL_INDEX);
        });
    }

    #[test]
    fn box_no_intersect() {
        full_box_check(|b: AABox| {
            let r = Ray::new(P(6.0, 0.0, 1.0), V(1.0, 1.0, 1.0));

            let res = b.intersect(&r);
            assert!(res.is_none());
        });
    }

    #[test]
    fn ray_from_box_face() {
        full_box_check(|b: AABox| {
            let r = Ray::new(P(5.0, 4.0, 3.0), V(1.0, 1.0, 1.0));

            let res = b.intersect(&r);
            assert!(res.is_none());
        });
    }

    #[test]
    fn box_from_inside() {
        full_box_check(|b: AABox| {
            let r = Ray::new(P(4.0, 3.0, 2.0), V(1.0, 1.0, 1.0));

            let res = b.intersect(&r);
            assert!(!res.is_none());

            let hit = res.unwrap();
            assert_vec3_eq(&hit.point, &P(5.0, 4.0, 3.0));
        });
    }

    #[test]
    fn box_perp_ray() {
        full_box_check(|b| {
            let r = Ray::new(P(4.0, 3.0, 2.0), V(1.0, 0.0, 0.0));

            let res = b.intersect(&r);
            assert!(!res.is_none());

            let hit = res.unwrap();
            assert_vec3_eq(&hit.point, &P(5.0, 3.0, 2.0));
        });
    }
}
