use crate::geometry::{Color, DEFAULT_EPSILON, Hit, Hittable, Ray, Scene, Vec3, colors, is_zero};

const SAMPLES_COUNT: u32 = 10;

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
                let direct_lighting = self.calc_direct_lighting(&hit);
                let indirect_lighting = self.calc_indirect_lighting(&hit, 1);

                return direct_lighting.add(&indirect_lighting);
            }
            None => colors::WHITE,
        }
    }

    /// calculate direct lighting for the hit
    fn calc_direct_lighting(&self, hit: &Hit) -> Color {
        let mut result = match &self.scene.ambient_light {
            Some(light) => hit.color.mult(&light.color),
            None => colors::BLACK,
        };

        let hit_pt = hit.point.add(&hit.normal.scale(DEFAULT_EPSILON));

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

                let colored_hit = hit.color.mult(&diffuse_light);

                result.add_mut(&colored_hit);
            }
        }

        result
    }

    fn calc_indirect_lighting(&self, hit: &Hit, hops: u32) -> Color {
        let mut result = colors::BLACK;

        if hops != 0 {
            let mut bounces_color = colors::BLACK;
            let hit_pt = hit.point.add(&hit.normal.scale(DEFAULT_EPSILON));

            for _ in 0..SAMPLES_COUNT {
                let bounce_dir = hit.normal.add(&Vec3::random_unit()).normalize();
                let bounced_ray = Ray::new(hit_pt, bounce_dir);

                if let Some(bounced_hit) = self.scene.intersect(&bounced_ray) {
                    let bounced_hit_color = self.calc_direct_lighting(&bounced_hit);
                    bounces_color.add_mut(&bounced_hit_color);
                }
            }
            bounces_color.div_mut(SAMPLES_COUNT as f64);

            let mixed_color = bounces_color.mult(&hit.color);
            result.add_mut(&mixed_color);
        }

        result
    }
}
