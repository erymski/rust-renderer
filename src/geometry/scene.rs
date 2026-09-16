use crate::geometry::{AmbientLight, DirectionalLight, Hit, Hittable, Ray};

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
    directional_lights: Vec<DirectionalLight>,
    pub ambient_light: Option<AmbientLight>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            directional_lights: Vec::new(),
            ambient_light: None,
        }
    }

    pub fn add_object<T>(&mut self, obj: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Box::new(obj));
    }

    pub fn add_light(&mut self, light: DirectionalLight) {
        self.directional_lights.push(light);
    }

    pub fn lights(&self) -> &Vec<DirectionalLight> {
        &self.directional_lights
    }
}

impl Hittable for Scene {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        for object in &self.objects {
            if let Some(candidate) = object.intersect(ray) {
                if hit.as_ref().is_none_or(|current| current.t > candidate.t) {
                    hit = Some(candidate);
                }
            }
        }

        hit
    }
}

#[cfg(test)]
mod tests {

    use crate::geometry::{
        Point3, Ray, Sphere,
        test_utils::{assert_approx_eq, assert_vec3_eq},
    };

    use super::*;

    #[test]
    fn empty_scene() {
        let scene = Scene::new();
        let ray = Ray::from_points(Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 0.0, 1.0));

        let hit = scene.intersect(&ray);
        assert!(hit.is_none());
    }

    #[test]
    fn hit_sphere() {
        let mut scene = Scene::new();
        scene.add_object(Sphere::blue(Point3::new(0.0, 0.0, 10.0), 5.0));
        let ray = Ray::from_points(Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 0.0, 1.0));

        let value = scene.intersect(&ray).expect("expected ray to hit sphere");
        assert_approx_eq(value.t, 5.0); // distance from the ray's start
        assert_vec3_eq(&value.point, &Point3::new(0.0, 0.0, 5.0));
        assert_vec3_eq(&value.normal, &Point3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn closest_sphere_independent_of_insertion_order() {
        let ray = Ray::from_points(Point3::new(0.0, 0.0, 0.0), &Point3::new(0.0, 0.0, 1.0));

        // pairs of (z, radius) for spheres. Same spheres, but in different insertion orders
        for spheres in [[(10.0, 5.0), (16.0, 3.0)], [(16.0, 3.0), (10.0, 5.0)]] {
            let mut scene = Scene::new();
            for (z, radius) in spheres {
                scene.add_object(Sphere::blue(Point3::new(0.0, 0.0, z), radius));
            }

            let value = scene.intersect(&ray).expect("expected ray to hit sphere");
            assert_approx_eq(value.t, 5.0);
        }
    }
}
