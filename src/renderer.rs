use crate::geometry::{
    Color, DEFAULT_EPSILON, Hit, Hittable, Lambertian, Material, Ray, Scene, colors, is_zero,
    random_unit,
};

const SAMPLES_COUNT: u32 = 7;
const BOUNCE_COUNT: u32 = 2;

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    scene: Scene,
}

impl Renderer {
    pub fn new(width: u32, height: u32, scene: Scene) -> Self {
        Renderer {
            width,
            height,
            scene,
        }
    }

    pub fn calc_point(&self, ray: &Ray) -> Color {
        match self.scene.intersect(ray) {
            Some(hit) => {
                let material = self.scene.get_material(hit.material_index);
                match material {
                    Material::Lambertian(lambertian) => {
                        let direct_lighting = self.calc_direct_lighting(&hit, &lambertian);
                        let indirect_lighting =
                            self.calc_indirect_lighting(&hit, &lambertian, BOUNCE_COUNT);

                        return direct_lighting + indirect_lighting;
                    }
                }
            }
            None => colors::WHITE,
        }
    }

    /// calculate direct lighting for the hit
    fn calc_direct_lighting(&self, hit: &Hit, material: &Lambertian) -> Color {
        let mut result = match &self.scene.ambient_light {
            Some(light) => material.albedo * light.color,
            None => colors::BLACK,
        };

        let hit_pt = hit.point + hit.normal.scale(DEFAULT_EPSILON);

        for light in self.scene.lights() {
            // check if the light is reachable
            let ray_to_light = Ray::new(hit_pt, light.direction.scale(-1.));
            let intersect_to_light = self.scene.intersect(&ray_to_light);

            // TODO: defect - exclude itself from intersections, otherwise we can miss a real hit
            if intersect_to_light.is_none_or(|hit| is_zero(hit.t)) {
                // nothing between light and the object
                let dir = light.direction.scale(-1.);
                let diffuse = dir.dot(&hit.normal).max(0.0);
                let diffuse_light = light.color.scale(diffuse);

                let colored_hit = material.albedo * diffuse_light;

                result += colored_hit;
            }
        }

        result
    }

    fn calc_indirect_lighting(&self, hit: &Hit, material: &Lambertian, hops: u32) -> Color {
        let mut bounces_color = colors::BLACK;
        let hit_pt = hit.point + hit.normal.scale(DEFAULT_EPSILON);

        for _ in 0..SAMPLES_COUNT {
            let bounce_dir = (hit.normal + random_unit()).normalize();
            let bounced_ray = Ray::new(hit_pt, bounce_dir);

            if let Some(bounced_hit) = self.scene.intersect(&bounced_ray) {
                let bounced_material = self.scene.get_material(bounced_hit.material_index);
                match bounced_material {
                    Material::Lambertian(lambertian) => {
                        let bounced_direct = self.calc_direct_lighting(&bounced_hit, lambertian);
                        bounces_color += bounced_direct;

                        if hops > 0 {
                            let bounced_indirect =
                                self.calc_indirect_lighting(&bounced_hit, lambertian, hops - 1);
                            bounces_color += bounced_indirect * lambertian.albedo;
                        }
                    }
                }
            }
        }
        // calc average color
        bounces_color /= SAMPLES_COUNT as f64;

        bounces_color *= material.albedo;
        bounces_color
    }
}
