#[derive(Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn scale(self, k: f64) -> Self {
        Vec2 {
            x: k * self.x,
            y: k * self.y,
        }
    }

    pub fn dist_to(&self, p: &Vec2) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::test_utils::assert_approx_eq;


    #[test]
    fn vec2_dist_to() {
        let v = Vec2::new(0.0, 0.0);
        let p = Vec2::new(1.0, 1.0);
        let dist = v.dist_to(&p);
        assert_approx_eq(dist, (2f64).sqrt());
    }
}
