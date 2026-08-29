mod camera;
pub(crate) mod colors;
mod hit;
mod ray;
mod scene;
mod sphere;
mod vec2;
mod vec3;

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

#[cfg(test)]
mod test_utils;

pub use camera::{Camera, Viewport};
pub use hit::Hit;
pub use ray::Ray;
pub use scene::Scene;
pub use sphere::Sphere;
pub use vec3::{Color, Point3, Vec3};
