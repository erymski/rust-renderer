use crate::geometry::{Hit3d, Hittable3d, Point3, Ray};

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

impl Hittable3d for Scene3d {
    fn hit(&self, ray: &Ray) -> Option<Hit3d> {
        let mut hit: Option<Hit3d> = None;
        for object in &self.objects {
            match object.hit(ray) {
                Some(candidate) => {
                    if hit.as_ref().is_none_or(|current| current.t > candidate.t) {
                        hit = Some(candidate);
                    }
                }
                None => continue,
            };
        }

        hit
    }
}

#[cfg(test)]
mod tests {

    use crate::geometry::{Point3, Sphere};

    use super::*;

    #[test]
    fn empty_scene() {
        let scene = Scene3d::new();
        let ray = Ray::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0));

        let hit = scene.hit(&ray);
        assert!(hit.is_none())
    }

    #[test]
    fn hit_sphere() {
        let mut scene = Scene3d::new();
        scene.add(Sphere::new(Point3::new(0.0, 0.0, 10.0), 5.0));
        let ray = Ray::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0));

        let hit = scene.hit(&ray);
        assert!(hit.is_some());

        let value = hit.unwrap();
        assert_eq!(value.t, 5.0) // distance from the ray's start
    }
}
