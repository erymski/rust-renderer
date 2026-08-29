pub(crate) mod colors;
pub(crate) mod hit;
pub(crate) mod ray;
pub(crate) mod scene;
pub(crate) mod sphere;
pub(crate) mod vec2;
pub(crate) mod vec3;

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

#[cfg(test)]
mod test_utils;

pub use hit::Hit;
pub use ray::Ray;
pub use scene::Scene;
pub use sphere::Sphere;
pub use vec3::{Color, Point3, Vec3};
