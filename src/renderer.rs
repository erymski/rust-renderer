use crate::geometry::{Color, DEFAULT_EPSILON, Hit, Hittable, Ray, Scene, Vec3, colors, is_zero};

const SAMPLES_COUNT: u32 = 10;

pub(crate) fn hit_for_scene(hit: &Hit, scene: &Scene, hops: u32) -> Color {
    let mut result = match &scene.ambient_light {
        Some(light) => hit.color.mult(&light.color),
        None => colors::BLACK,
    };

    let hit_pt = hit.point.add(&hit.normal.scale(DEFAULT_EPSILON));

    for light in scene.lights() {
        // check if the light is reachable
        let ray_to_light = Ray::new(hit_pt, light.direction.scale(-1.));
        let intersect_to_light = scene.intersect(&ray_to_light);

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

    if hops != 0 {
        let mut bounces_color = colors::BLACK;
        for _ in 0..SAMPLES_COUNT {
            let bounce_dir = hit.normal.add(&Vec3::random_unit()).normalize();
            let bounced_ray = Ray::new(hit_pt, bounce_dir);

            if let Some(bounced_hit) = scene.intersect(&bounced_ray) {
                bounces_color.add_mut(&bounced_hit.color);
            }
        }
        bounces_color.div_mut(SAMPLES_COUNT as f64);

        result.add_mut(&bounces_color.mult(&hit.color));
    }

    result
}

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
            Some(hit) => hit_for_scene(&hit, &self.scene, 1),
            None => colors::WHITE,
        }
    }
}
