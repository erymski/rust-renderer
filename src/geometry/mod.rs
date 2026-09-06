mod camera;
pub(crate) mod colors;
mod hit;
mod lights;
mod ray;
mod scene;
mod sphere;
mod tone_mapping;
mod vec2;
mod vec3;

pub(crate) trait Hittable {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

pub(crate) trait Primitive: Hittable {
    fn color(&self) -> vec3::Color;
}

#[cfg(test)]
mod test_utils;

pub use camera::{Camera, Viewport};
pub use hit::Hit;
pub use lights::{AmbientLight, DirectionalLight};
pub use ray::Ray;
pub use scene::Scene;
pub use sphere::Sphere;
pub use tone_mapping::{ToneMapper, color_identity};
pub use vec3::{Color, Point3, Vec3};
