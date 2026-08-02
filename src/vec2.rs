pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn scale(k: f64, v: &Vec2) -> Self {
        Vec2 {
            x: k * v.x,
            y: k * v.y,
        }
    }
}
