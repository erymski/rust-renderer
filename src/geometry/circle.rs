use crate::geometry::{Color, Hittable2d, Vec2};

pub(crate) struct Circle {
    center: Vec2,
    radius: f64,
    color: Color,
}

impl Circle {
    pub(crate) fn new(center: Vec2, radius: f64, color: Color) -> Self {
        Circle {
            center,
            radius,
            color,
        }
    }
}

impl Hittable2d for Circle {
    fn hit(&self, pt: &Vec2) -> Option<Color> {
        let dx = pt.x - self.center.x;
        let dy = pt.y - self.center.y;
        if dx * dx + dy * dy <= self.radius * self.radius {
            Some(self.color)
        } else {
            None
        }
    }
}
