pub(crate) mod vec2;
pub(crate) mod vec3;
pub(crate) mod ray;
pub(crate) mod rect2d;
pub(crate) mod circle;
pub(crate) mod scene2d;
pub(crate) mod colors;

pub(crate) trait Hittable2d {
    fn hit(&self, pt: &Vec2) -> Option<Color>;
}

#[cfg(test)]
mod test_utils;

pub use vec3::{Color, Point3, Vec3};
pub use vec2::Vec2;
pub use ray::Ray;
pub(crate) use circle::Circle;
pub(crate) use rect2d::Rect2d;
pub(crate) use scene2d::Scene2d;
