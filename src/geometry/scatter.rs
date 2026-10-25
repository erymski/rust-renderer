use crate::geometry::{Color, Hit, Ray};

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Scatterized {
    fn build_scatter(&self, incoming_ray: &Ray, hit: &Hit) -> Option<Scatter>;
}
