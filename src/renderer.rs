use crate::geometry::{Circle, Color, Hittable2d, Rect2d, Scene2d, Vec2, Vec3};

#[allow(dead_code)]
pub const WHITE: Color = Color {
    x: 255.,
    y: 255.,
    z: 255.,
};

#[allow(dead_code)]
pub struct Renderer {
    pub width: usize,
    pub height: usize,
    scene: Scene2d,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut renderer = Renderer {
            width,
            height,
            scene: Scene2d::new(),
        };

        renderer.scene.add_object(Rect2d::new(
            Vec2::new(0.2, 0.8),
            Vec2::new(0.2, 0.8),
            Color::new(128., 54., 204.),
        ));

        renderer.scene.add_object(Circle::new(
            Vec2::new(0.1, 0.3),
            0.25,
            Color::new(255., 0., 0.),
        ));

        renderer
    }

    pub fn calc_point(&self, x: u32, y: u32) -> Color {
        let color = match self.scene.hit(&Vec2::new(
            x as f64 / self.width as f64,
            y as f64 / self.height as f64,
        )) {
            Some(color) => color,
            None => self.sky_gradient(x, y),
        };
        color
    }

    #[allow(dead_code)]
    fn simple_gradient(&self, x: u32, y: u32) -> Vec3 {
        let r = x as f64 / self.width as f64 * 255.0;
        let g = y as f64 / self.height as f64 * 255.0;
        let b = 128f64;
        Color::new(r as f64, g, b as f64)
    }

    fn sky_gradient(&self, _x: u32, y: u32) -> Vec3 {

        const WHITE: Color = Color {
            x: 255.,
            y: 255.,
            z: 255.,
        };

        const BLUE: Color = Color {
            x: 128.,
            y: 192.,
            z: 255.,
        };

        let a = y as f64 / self.height as f64;

        BLUE.scale(1.0 - a).add(&WHITE.scale(a))
    }
}
