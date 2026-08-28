use crate::geometry::{
    Color, Hittable3d, Point3, Ray, Scene3d,
    colors::{BLUE, WHITE},
};

#[allow(dead_code)]
pub struct Renderer {
    pub width: usize,
    pub height: usize,
    scene: Scene3d,
}

impl Renderer {
    pub fn new(width: usize, height: usize, scene: Scene3d) -> Self {
        Renderer {
            width,
            height,
            scene,
        }
    }

    pub fn calc_point(&self, _x: u32, _y: u32) -> Color {
        let ray = Ray::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0));

        let color = match self.scene.hit(&ray) {
            Some(_color) => BLUE,
            None => WHITE,
        };
        color
    }
}
