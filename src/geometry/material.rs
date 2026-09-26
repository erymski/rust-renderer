use crate::geometry::Color;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

#[derive(Debug)]
pub enum Material {
    Lambertian(Lambertian),
}
