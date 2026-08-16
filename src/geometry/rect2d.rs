use crate::geometry::{Color, Hittable2d, Vec2};

pub(crate) struct Rect2d {
    u_bounds: Vec2,
    v_bounds: Vec2,
    color: Color,
}

impl Rect2d {
    pub(crate) fn new(u_bounds: Vec2, v_bounds: Vec2, color: Color) -> Self {
        Rect2d {
            u_bounds,
            v_bounds,
            color,
        }
    }
}

impl Hittable2d for Rect2d {
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
