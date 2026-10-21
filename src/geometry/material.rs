use crate::geometry::{
    Color, Hit, Ray, random_unit, reflect,
    scatter::{Scatter, Scatterized},
};

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Scatterized for Lambertian {
    fn build_scatter(&self, _incoming_ray: &Ray, hit: &Hit) -> super::scatter::Scatter {
        let bounce_dir = (hit.normal + random_unit()).normalize();
        let hit_pt = hit.calc_offset();

        let bounced_ray = Ray::new(hit_pt, bounce_dir);
        Scatter {
            attenuation: self.albedo,
            ray: bounced_ray,
        }
    }
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

impl Scatterized for Metal {
    fn build_scatter(&self, incoming_ray: &Ray, hit: &Hit) -> Scatter {
        let reflected_v = reflect(incoming_ray.dir, hit.normal);
        let reflected_ray = Ray::new(hit.calc_offset(), reflected_v); // TODO: use roughness

        Scatter {
            ray: reflected_ray,
            attenuation: self.color,
        }
    }
}

#[derive(Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}
