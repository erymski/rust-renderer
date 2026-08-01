use crate::vec3::Color;

pub struct Renderer {
    pub width: usize,
    pub height: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Renderer { width, height }
    }

    pub fn calc_point(&self, x: u32, y: u32) -> Color {
        let r = x as f64 / self.width as f64 * 255.0;
        let g = y as f64 / self.height as f64 * 255.0;
        let b = 128f64;
        Color::new(r as f64, g, b as f64)
    }
}