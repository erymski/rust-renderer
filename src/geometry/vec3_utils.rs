use crate::geometry::{DEFAULT_EPSILON, Point3};
use crate::geometry::{V, Vec3};
use rand::random_range;

/// generate random unit vector
pub fn random_unit() -> Vec3 {
    loop {
        let mut res = V(
            random_range(-1.0..1.0),
            random_range(-1.0..1.0),
            random_range(-1.0..1.0),
        );

        // generate vector until it's inside unit sphere
        let len_squared = res.length_squared();
        if len_squared > DEFAULT_EPSILON && len_squared <= 1.0 {
            res.div_mut(len_squared.sqrt());
            return res;
        }
    }
}

/// reflect v against n (normal)
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    let v_projected_to_n = n * v.dot(&n);
    v - v_projected_to_n * 2.0
}

/// make a slightly offset point, which moved from the point in normal direction
pub fn make_offset(p: Point3, normal: Vec3) -> Point3 {
    p + normal.scale(DEFAULT_EPSILON)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::geometry::{
        P,
        test_utils::{assert_approx_eq, assert_vec3_eq, assert_vec3_neq},
    };

    #[test]
    fn vec3_random() {
        let v1 = random_unit();
        assert_approx_eq(v1.length_squared(), 1.0);
        let v2 = random_unit();
        assert_approx_eq(v2.length_squared(), 1.0);

        assert_vec3_neq(&v1, &v2);
    }

    #[test]
    fn vec3_reflection() {
        let v = V(1.0, -1.0, 0.0);
        let n = V(0.0, 1.0, 0.0);

        let r = reflect(v, n);

        assert_vec3_eq(&r, &V(1.0, 1.0, 0.0));
    }

    #[test]
    fn offset_is_in_normal_direction() {
        let p = P(1., 2., -15.);
        let normal = V(3.36, -8.3, 21.55).normalize();
        let offset = make_offset(p, normal);

        let dir = (offset - p).normalize();
        assert_approx_eq(dir.dot(&normal), 0.0);
    }
}
