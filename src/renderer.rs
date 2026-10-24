use crate::geometry::{
    Color, Hit, Hittable, Lambertian, Material, Ray, RenderingContext, Scene, colors, is_zero,
    make_offset, random_unit, reflect,
};

const SAMPLES_COUNT: u32 = 5;
const BOUNCE_COUNT: u32 = 3;

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    scene: Scene,
}

impl Renderer {
    pub fn new(width: u32, height: u32, scene: Scene) -> Self {
        Renderer {
            width,
            height,
            scene,
        }
    }

    pub fn go(&self, ray: &Ray, context: &RenderingContext) -> Color {
        let mut bounces_color = colors::BLACK;
        for _ in 0..SAMPLES_COUNT {
            bounces_color += self.trace(ray, BOUNCE_COUNT, context)
        }
        bounces_color / SAMPLES_COUNT as f64
    }

    fn trace(&self, ray: &Ray, hops: u32, context: &RenderingContext) -> Color {
        match self.scene.intersect(ray) {
            None => Color::new(0.25, 0.37, 0.5),
            Some(hit) => {
                let dbg = false; //(context.x == 0) && (context.y == context.height - 1);

                let material = self.scene.get_material(hit.material_index);
                match material {
                    Material::Lambertian(lambertian) => {
                        if dbg {
                            println!("{:?}", lambertian)
                        }

                        let direct = self.calc_direct_lighting(&hit, &lambertian);

                        if dbg {
                            println!("direct: {:?}", direct)
                        }

                        if hops == 0 {
                            return direct;
                        }

                        let bounce_dir = (hit.normal + random_unit()).normalize();
                        let hit_pt = hit.calc_offset();

                        let bounced_ray = Ray::new(hit_pt, bounce_dir);
                        let indirect = self.trace(&bounced_ray, hops - 1, context);

                        if dbg {
                            println!("indirect: {:?}", indirect);
                            println!("total: {:?}", direct + (indirect * lambertian.albedo));
                        }
                        direct + (indirect * lambertian.albedo)
                    }
                    Material::Metal(_metal) => colors::BLACK, // metal.color,
                }
            }
        }
    }

    pub fn calc_point(&self, ray: &Ray) -> Color {
        match self.scene.intersect(ray) {
            Some(hit) => {
                let material = self.scene.get_material(hit.material_index);
                match material {
                    Material::Lambertian(lambertian) => {
                        let direct_lighting = self.calc_direct_lighting(&hit, &lambertian);

                        let bounce_dir = (hit.normal + random_unit()).normalize();
                        let hit_pt = make_offset(hit.point, hit.normal);

                        let bounced_ray = Ray::new(hit_pt, bounce_dir);

                        let indirect_lighting =
                            self.calc_indirect_lighting(&hit, &ray, BOUNCE_COUNT);

                        direct_lighting + (indirect_lighting * lambertian.albedo)
                    }
                    Material::Metal(metal) => {
                        let reflected_v = reflect(ray.dir, hit.normal);
                        let reflected_ray = Ray::new(hit.calc_offset(), reflected_v);

                        let indirect =
                            self.calc_indirect_lighting(&hit, &reflected_ray, BOUNCE_COUNT);

                        metal.color * indirect
                    }
                }
            }
            None => colors::WHITE,
        }
    }

    /// calculate direct lighting for the hit
    fn calc_direct_lighting(&self, hit: &Hit, material: &Lambertian) -> Color {
        let mut result = colors::BLACK;

        let hit_pt = hit.calc_offset();

        for light in self.scene.lights() {
            // check if the light is reachable
            let ray_to_light = Ray::new(hit_pt, light.direction.scale(-1.));
            let intersect_to_light = self.scene.intersect(&ray_to_light);

            // TODO: defect - exclude itself from intersections, otherwise we can miss a real hit
            // if intersect_to_light.is_none_or(|hit2| is_zero(hit2.t)) {
            if intersect_to_light.is_none() {
                // nothing between light and the object
                let dir = light.direction.scale(-1.);
                let diffuse = dir.dot(&hit.normal).max(0.0);
                let diffuse_light = light.color.scale(diffuse);

                let colored_hit = material.albedo * diffuse_light;

                result += colored_hit;
            }
        }

        result
    }

    fn calc_indirect_lighting(&self, hit: &Hit, ray: &Ray, hops: u32) -> Color {
        let mut bounces_color = colors::BLACK;
        let hit_pt = make_offset(hit.point, hit.normal);

        for _ in 0..SAMPLES_COUNT {
            let bounce_dir = (hit.normal + random_unit()).normalize();
            let bounced_ray = Ray::new(hit_pt, bounce_dir);

            if let Some(bounced_hit) = self.scene.intersect(&bounced_ray) {
                let bounced_material = self.scene.get_material(bounced_hit.material_index);
                match bounced_material {
                    Material::Lambertian(lambertian) => {
                        let bounced_direct = self.calc_direct_lighting(&bounced_hit, lambertian);
                        bounces_color += bounced_direct;

                        if hops > 0 {
                            let bounced_indirect =
                                self.calc_indirect_lighting(&bounced_hit, &bounced_ray, hops - 1);
                            bounces_color += bounced_indirect * lambertian.albedo;
                        }
                    }
                    Material::Metal(metal) => {
                        let reflected_v = reflect(ray.dir, hit.normal);
                        let reflected_ray = Ray::new(hit.calc_offset(), reflected_v);

                        if hops > 0 {
                            let bounced_indirect =
                                self.calc_indirect_lighting(&bounced_hit, &reflected_ray, hops - 1);
                            bounces_color += bounced_indirect * metal.color;
                        }
                    }
                }
            }
        }
        // calc average color
        bounces_color /= SAMPLES_COUNT as f64;

        bounces_color
    }
}
