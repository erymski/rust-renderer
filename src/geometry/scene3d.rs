use crate::geometry::Hittable3d;

pub struct Scene3d {
    objects: Vec<Box<dyn Hittable3d>>,
}

impl Scene3d {
    pub fn new() -> Self {
        Scene3d {
            objects: Vec::new(),
        }
    }

    pub fn add<T>(&mut self, obj: T)
    where
        T: Hittable3d + 'static,
    {
        self.objects.push(Box::new(obj));
    }
}
