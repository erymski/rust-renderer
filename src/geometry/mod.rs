mod camera;
pub(crate) mod colors;
mod hit;
mod lights;
mod plane;
mod ray;
mod scene;
mod sphere;
mod tolerance;
mod tone_mapping;
mod vec2;
mod vec3;
mod vec3_ops;
mod vec3_utils;

use std::fmt::Debug;

pub(crate) trait Hittable: Debug {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

pub(crate) trait Primitive: Hittable {
    fn color(&self) -> vec3::Color;
}

pub struct RenderingSet {
    pub scene: Scene,
    pub camera: Camera,
}

#[cfg(test)]
mod test_utils;

pub use camera::{Camera, Viewport};
pub use hit::Hit;
pub use lights::{AmbientLight, DirectionalLight};
pub use plane::Plane;
pub use ray::Ray;
pub use scene::Scene;
pub use sphere::Sphere;
pub use tolerance::{
    DEFAULT_EPSILON, approx_eq, approx_eq_custom, is_zero, vec3_eq, vec3_eq_custom,
};
pub use tone_mapping::{ToneMapper, color_clamp, color_identity, color_reinhard};
pub use vec3::{C, Color, P, Point3, UNIT_X, UNIT_Y, UNIT_Z, V, Vec3};
pub use vec3_utils::random_unit;
