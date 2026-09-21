use super::vec3::Vec3;

pub const DEFAULT_EPSILON: f64 = 1e-10;

/// check if float is about zero
pub const fn is_zero(a: f64) -> bool {
    approx_eq(a, 0.0)
}

pub const fn approx_eq(a: f64, b: f64) -> bool {
    approx_eq_custom(a, b, DEFAULT_EPSILON)
}

pub const fn approx_eq_custom(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

pub fn vec3_eq(a: &Vec3, b: &Vec3) -> bool {
    vec3_eq_custom(a, b, DEFAULT_EPSILON)
}

pub fn vec3_eq_custom(a: &Vec3, b: &Vec3, epsilon: f64) -> bool {
    let distance = a.dist_to(b); // TODO: no need sqrt, can be faster
    distance < epsilon
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::V;

    #[test]
    fn eq_same_vectors() {
        let v = Vec3 {
            x: 0.,
            y: 1.,
            z: 2.,
        };
        assert!(vec3_eq(&v, &v));
    }

    #[test]
    fn neq_vectors() {
        let v1 = V(1., 2., 3.);
        let v2 = V(4., 2., 3.);
        assert!(!vec3_eq(&v1, &v2));
    }

    // TODO: Add more tests for tolerance
}
