use std::ops::Neg;

use crate::geometry::{Color, Hittable, Ray, Scene, colors};

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
            Some(hit) => {
                let diffuse = self
                    .scene
                    .directional_light
                    .direction
                    .dot(&hit.normal)
                    .neg();

                self.scene
                    .directional_light
                    .color
                    .mult(&hit.color)
                    .scale(diffuse)
            }
            None => colors::WHITE,
        };
        hit
    }
}
