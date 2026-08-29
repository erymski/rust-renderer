use crate::geometry::{Color, Hittable, Ray, Scene, colors::WHITE};

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
            Some(hit) => hit.color,
            None => WHITE,
        };
        hit
    }
}
