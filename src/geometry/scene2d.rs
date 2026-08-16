use crate::geometry::{Color, Hittable2d, Vec2};

pub(crate) struct Scene2d {
    objects: Vec<Box<dyn Hittable2d>>,
}

impl Scene2d {
    pub(crate) fn new() -> Self {
        Scene2d {
            objects: Vec::new(),
        }
    }

    pub(crate) fn add_object<T>(&mut self, object: T)
    where
        T: Hittable2d + 'static,
    {
        self.objects.push(Box::new(object));
    }
}

impl Hittable2d for Scene2d {
    fn hit(&self, pt: &Vec2) -> Option<Color> {
        for object in &self.objects {
            if let Some(color) = object.hit(pt) {
                return Some(color);
            }
        }
        None
    }
}
