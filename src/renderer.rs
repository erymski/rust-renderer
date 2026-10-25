use crate::geometry::{Color, Hit, Hittable, Ray, RenderingContext, Scatterized, Scene, colors};

const SAMPLES_COUNT: u32 = 4;
const BOUNCE_COUNT: u32 = 4;

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

    pub fn go(&self, ray: &Ray, context: &RenderingContext) -> Color {
        let mut bounces_color = colors::BLACK;
        for _ in 0..SAMPLES_COUNT {
            bounces_color += self.trace(ray, BOUNCE_COUNT, context)
        }
        bounces_color / SAMPLES_COUNT as f64
    }

    fn trace(&self, ray: &Ray, hops: u32, context: &RenderingContext) -> Color {
        match self.scene.intersect(ray) {
            None => Color::new(0.25, 0.37, 0.5),
            Some(hit) => {
                let material = self.scene.get_material(hit.material_index);
                let scatter = material.build_scatter(ray, &hit);
                let mut lighting = self.calc_direct_lighting(&hit) * scatter.attenuation;
                if hops > 0 {
                    lighting += self.trace(&scatter.ray, hops - 1, context);
                }

                lighting * scatter.attenuation
            }
        }
    }

    /// calculate direct lighting for the hit
    fn calc_direct_lighting(&self, hit: &Hit) -> Color {
        let mut result = colors::BLACK;

        let hit_pt = hit.calc_offset();

        for light in self.scene.lights() {
            // check if the light is reachable
            let ray_to_light = Ray::new(hit_pt, light.direction.scale(-1.));
            let intersect_to_light = self.scene.intersect(&ray_to_light);

            // TODO: defect - exclude itself from intersections, otherwise we can miss a real hit
            // if intersect_to_light.is_none_or(|hit2| is_zero(hit2.t)) {
            if intersect_to_light.is_none() {
                // nothing between light and the object
                let dir = light.direction.scale(-1.);
                let diffuse = dir.dot(&hit.normal).max(0.0);
                let diffuse_light = light.color.scale(diffuse);

                result += diffuse_light;
            }
        }

        result
    }
}
