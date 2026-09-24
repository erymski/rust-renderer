use crate::geometry::DEFAULT_EPSILON;
use crate::geometry::{V, Vec3};
use rand::random_range;

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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::geometry::test_utils::{assert_approx_eq, assert_vec3_neq};

    #[test]
    fn vec3_random() {
        let v1 = random_unit();
        assert_approx_eq(v1.length_squared(), 1.0);
        let v2 = random_unit();
        assert_approx_eq(v2.length_squared(), 1.0);

        assert_vec3_neq(&v1, &v2);
    }
}
