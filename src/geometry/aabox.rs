use crate::geometry::{Hit, Hittable, Point3, Ray, V, is_zero};

/** Axis aligned box */
#[derive(Debug, Clone, Copy)]
pub struct AABox {
    pub min: Point3,
    pub max: Point3,
    material_index: usize,
}

impl Hittable for AABox {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut min_t = f64::MIN;
        let mut max_t = f64::MAX;

        let dir = ray.dir();

        for i in 0..3 {
            let dir = dir[i];
            if is_zero(dir) {
                todo!("implement it")
            }

            // looking for *maximum* of minimums
            let mut curr_min_t = (self.min[i] - ray.from[i]) / dir;
            let mut curr_max_t = (self.max[i] - ray.from[i]) / dir;
            if curr_max_t < curr_min_t {
                std::mem::swap(&mut curr_min_t, &mut curr_max_t);
            }

            if curr_min_t > min_t {
                min_t = curr_min_t;
            }

            // looking for *minimum* of maximuses
            if curr_max_t < max_t {
                max_t = curr_max_t;
            }

            println!("{}: {}, {}", i, curr_min_t, curr_max_t)
        }

        println!("{}, {}", min_t, max_t);

        if min_t > max_t {
            return None;
        }

        if min_t < 0.0 && max_t < 0.0 {}

        let t = if min_t < 0.0 { max_t } else { min_t };

        // ray is going away of the box
        if t < 0.0 {
            return None;
        }

        let intersection = ray.from + dir * t;

        Some(Hit {
            point: intersection,
            normal: V(0.0, 0.0, 0.0),
            t: t,
            material_index: self.material_index,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::geometry::{P, V, test_utils::assert_vec3_eq};

    use super::*;

    #[test]
    fn box_diagonal() {
        let b = AABox {
            min: P(0.0, 0.0, 0.0),
            max: P(5.0, 5.0, 5.0),
            material_index: 3,
        };

        let r = Ray::new(P(6.0, 0.0, 1.0), V(-1.0, 1.0, 1.0));

        let res = b.intersect(&r);
        assert!(!res.is_none());

        let hit = res.unwrap();
        assert_vec3_eq(&hit.point, &P(5.0, 1.0, 2.0));
    }
}
