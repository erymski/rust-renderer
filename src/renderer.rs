use crate::geometry::{
    Color, Hittable, Point3, Ray, Scene,
    colors::{BLUE, WHITE},
};

#[allow(dead_code)]
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
        let color = match self.scene.hit(&ray) {
            Some(_color) => BLUE,
            None => WHITE,
        };
        color
    }
}
