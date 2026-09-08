use std::ops::Neg;

use crate::geometry::{Color, Hit, Hittable, Ray, Scene, colors};

pub(crate) fn hit_for_scene(hit: &Hit, scene: &Scene) -> Color {
    let lights = scene.lights();
    if lights.is_empty() {
        return colors::BLACK;
    }

    let mut result = colors::BLACK;
    for light in lights {
        let diffuse = light.direction.dot(&hit.normal).neg();
        let diffuse_light = light.color.scale(diffuse);

        let colored_hit = hit.color.mult(&diffuse_light);

        result.add_mut(&colored_hit);
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
        let hit = match self.scene.intersect(&ray) {
            Some(hit) => hit_for_scene(&hit, &self.scene),
            None => colors::WHITE,
        };
        hit
    }
}
