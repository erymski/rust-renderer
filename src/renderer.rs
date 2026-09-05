use crate::geometry::{
    Color, DirectionalLight, Hittable, Ray, Scene,
    colors::{BLACK, WHITE, lerp},
};

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    scene: Scene,
    directional_light: DirectionalLight,
}

impl Renderer {
    pub fn new(width: u32, height: u32, scene: Scene, directional_light: DirectionalLight) -> Self {
        Renderer {
            width,
            height,
            scene,
            directional_light,
        }
    }

    pub fn calc_point(&self, ray: &Ray) -> Color {
        let hit = match self.scene.intersect(&ray) {
            Some(hit) => {
                let dot = self.directional_light.direction.dot(&hit.normal);
                if dot > 0.0 {
                    return BLACK;
                }

                lerp(-dot, &BLACK, &hit.color)
            }
            None => WHITE,
        };
        hit
    }
}
