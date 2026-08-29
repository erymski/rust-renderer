use crate::geometry::{
    Color, Hittable, Ray, Scene,
    colors::{BLACK, GREEN, WHITE, lerp},
};

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
                let dot = ray.dir.dot(&hit.normal);
                if dot > 0.0 {
                    return GREEN;
                }

                lerp(-dot, &BLACK, &hit.color)
            }
            None => WHITE,
        };
        hit
    }
}
