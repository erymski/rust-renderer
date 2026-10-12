use crate::geometry::Color;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

#[derive(Debug)]
pub struct Metal {
    pub color: Color,
    pub roughness: f64,
}

#[derive(Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}
