use crate::vec3::Color;

use crate::vec2::Vec2;

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

trait Hittable {
    fn hit(&self, pt: &Vec2) -> Option<Color>;
}

struct Rect2d {
    u_bounds: Vec2,
    v_bounds: Vec2,
    color: Color,
}

impl Rect2d {
    fn new(u_bounds: Vec2, v_bounds: Vec2, color: Color) -> Self {
        Rect2d {
            u_bounds,
            v_bounds,
            color,
        }
    }
}

impl Hittable for Rect2d {
    fn hit(&self, pt: &Vec2) -> Option<Color> {
        if pt.x >= self.u_bounds.x
            && pt.x <= self.u_bounds.y
            && pt.y >= self.v_bounds.x
            && pt.y <= self.v_bounds.y
        {
            Some(self.color)
        } else {
            None
        }
    }
}

struct Scene {
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    fn add_object<T>(&mut self, object: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for Scene {
    fn hit(&self, pt: &Vec2) -> Option<Color> {
        for object in &self.objects {
            if let Some(color) = object.hit(&pt) {
                return Some(color);
            }
        }
        None
    }
}

#[allow(dead_code)]
pub struct Renderer {
    pub width: usize,
    pub height: usize,
    scene: Scene,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut renderer = Renderer {
            width,
            height,
            scene: Scene::new(),
        };

        renderer.scene.add_object(Rect2d::new(
            Vec2::new(0.2, 0.8),
            Vec2::new(0.2, 0.8),
            Color::new(128., 54., 204.),
        ));
        renderer
    }

    pub fn calc_point(&self, x: u32, y: u32) -> Color {
        self.scene
            .hit(&Vec2::new(
                x as f64 / self.width as f64,
                y as f64 / self.height as f64,
            ))
            .unwrap_or(WHITE)

        // let u = x as f64 / self.width as f64;
        // let v = y as f64 / self.height as f64;

        // if u > 0.2 && u < 0.8 && v > 0.2 && v < 0.8 {
        //     BLACK
        // } else {
        //     WHITE
        // }

        //self.simple_gradient(x, y)
    }

    #[allow(dead_code)]
    fn simple_gradient(&self, x: u32, y: u32) -> crate::vec3::Vec3 {
        let r = x as f64 / self.width as f64 * 255.0;
        let g = y as f64 / self.height as f64 * 255.0;
        let b = 128f64;
        Color::new(r as f64, g, b as f64)
    }
}
