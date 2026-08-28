pub(crate) mod circle;
pub(crate) mod colors;
pub(crate) mod hit3d;
pub(crate) mod ray;
pub(crate) mod rect2d;
pub(crate) mod scene2d;
pub(crate) mod scene3d;
pub(crate) mod sphere;
pub(crate) mod vec2;
pub(crate) mod vec3;

pub(crate) trait Hittable2d {
    fn hit(&self, pt: &Vec2) -> Option<Color>;
}

pub(crate) trait Hittable3d {
    fn hit(&self, ray: &Ray) -> Option<Hit3d>;
}

#[cfg(test)]
mod test_utils;

pub(crate) use circle::Circle;
pub use hit3d::Hit3d;
pub use ray::Ray;
pub(crate) use rect2d::Rect2d;
pub(crate) use scene2d::Scene2d;
pub use sphere::Sphere;
pub use vec2::Vec2;
pub use vec3::{Color, Point3, Vec3};
