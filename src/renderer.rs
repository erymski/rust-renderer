use crate::geometry::{colors::{lerp, BLUE, WHITE}, Color, Hittable2d, Scene2d, Vec2};

#[allow(dead_code)]
pub struct Renderer {
    pub width: usize,
    pub height: usize,
    scene: Scene2d,
}

impl Renderer {
    pub fn new(width: usize, height: usize, scene: Scene2d) -> Self {
        Renderer {
            width,
            height,
            scene,
        }
    }

    pub fn calc_point(&self, x: u32, y: u32) -> Color {
        let color = match self.scene.hit(&Vec2::new(
            x as f64 / self.width as f64,
            y as f64 / self.height as f64,
        )) {
            Some(color) => color,
            None => self.sky_gradient(y),
        };
        color
    }

    fn sky_gradient(&self, y: u32) -> Color {
        let a = y as f64 / self.height as f64;

        lerp(a, &BLUE, &WHITE)
    }

}
