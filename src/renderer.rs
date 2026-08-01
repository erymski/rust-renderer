use crate::vec3::Color;

pub const BLACK: Color = Color {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const WHITE: Color = Color {
    x: 255.,
    y: 255.,
    z: 255.,
};

#[allow(dead_code)]
pub struct Renderer {
    pub width: usize,
    pub height: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Renderer { width, height }
    }

    pub fn calc_point(&self, x: u32, y: u32) -> Color {

        let u = x as f64 / self.width as f64;
        let v = y as f64 / self.height as f64;

        if u > 0.2 && u < 0.8 && v > 0.2 && v < 0.8 {
            BLACK
        } else {
            WHITE
        }

        //self.simple_gradient(x, y)
    }

    fn simple_gradient(&self, x: u32, y: u32) -> crate::vec3::Vec3 {
        let r = x as f64 / self.width as f64 * 255.0;
        let g = y as f64 / self.height as f64 * 255.0;
        let b = 128f64;
        Color::new(r as f64, g, b as f64)
    }
}