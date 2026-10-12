use crate::geometry::Color;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

#[derive(Debug)]
pub struct Metal {
    pub color: Color,
    /// Metal roughness 0..1
    /// 0.0 - perfect metal
    /// 0.1 - very smooth
    /// 0.3 - slightly rough
    /// 0.6 - rough
    /// 1.0 - very rough
    pub roughness: f64,
}

#[derive(Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}
