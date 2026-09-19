use crate::geometry::{DEFAULT_EPSILON, approx_eq_custom, vec3_eq_custom};

use super::vec2::Vec2;
use super::vec3::Vec3;

pub fn assert_vec3_eq(a: &Vec3, b: &Vec3) {
    assert_vec3_eq_custom(a, b, DEFAULT_EPSILON);
}
pub fn assert_vec3_eq_custom(a: &Vec3, b: &Vec3, epsilon: f64) {
    assert!(
        vec3_eq_custom(a, b, epsilon),
        "points are not equal: {a:?} vs {b:?}"
    );
}

pub fn assert_vec3_neq(a: &Vec3, b: &Vec3) {
    assert_vec3_neq_custom(a, b, DEFAULT_EPSILON);
}
pub fn assert_vec3_neq_custom(a: &Vec3, b: &Vec3, epsilon: f64) {
    assert!(
        !vec3_eq_custom(a, b, epsilon),
        "points are not equal: {a:?} vs {b:?}"
    );
}

pub fn assert_vec2_eq(a: &Vec2, b: &Vec2) {
    assert_vec2_eq_custom(a, b, DEFAULT_EPSILON);
}
pub fn assert_vec2_eq_custom(a: &Vec2, b: &Vec2, epsilon: f64) {
    let distance = a.dist_to(b);
    assert!(distance < epsilon, "points are not equal: {a:?} vs {b:?}");
}

pub fn assert_approx_eq(a: f64, b: f64) {
    assert_approx_eq_custom(a, b, DEFAULT_EPSILON);
}

pub fn assert_approx_eq_custom(a: f64, b: f64, epsilon: f64) {
    assert!(
        approx_eq_custom(a, b, epsilon),
        "values are not equal: {a} vs {b}"
    );
}
