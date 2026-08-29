use crate::geometry::{Hit, Hittable, Ray};

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn add<T>(&mut self, obj: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for Scene {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        for object in &self.objects {
            match object.intersect(ray) {
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

    use crate::geometry::{
        Point3, Sphere,
        test_utils::{assert_approx_eq, assert_vec3_eq},
    };

    use super::*;

    #[test]
    fn empty_scene() {
        let scene = Scene::new();
        let ray = Ray::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0));

        let hit = scene.intersect(&ray);
        assert!(hit.is_none())
    }

    #[test]
    fn hit_sphere() {
        let mut scene = Scene::new();
        scene.add(Sphere::blue(Point3::new(0.0, 0.0, 10.0), 5.0));
        let ray = Ray::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0));

        let value = scene.intersect(&ray).expect("expected ray to hit sphere");
        assert_approx_eq(value.t, 5.0); // distance from the ray's start
        assert_vec3_eq(&value.point, &Point3::new(0.0, 0.0, 5.0));
        assert_vec3_eq(&value.normal, &Point3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn closest_sphere_independent_of_insertion_order() {
        let ray = Ray::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0));

        // pairs of (z, radius) for spheres. Same spheres, but in different insertion orders
        for spheres in [[(10.0, 5.0), (16.0, 3.0)], [(16.0, 3.0), (10.0, 5.0)]] {
            let mut scene = Scene::new();
            for (z, radius) in spheres {
                scene.add(Sphere::blue(Point3::new(0.0, 0.0, z), radius));
            }

            let value = scene.intersect(&ray).expect("expected ray to hit sphere");
            assert_approx_eq(value.t, 5.0);
        }
    }
}
