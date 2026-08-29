use image::{Rgb, RgbImage};

mod geometry;

mod renderer;

use geometry::{Camera, Color, Scene, Sphere, Vec3};

use crate::geometry::{Point3, Ray};

fn build_scene() -> Scene {
    let mut scene = Scene::new();
    scene.add(Sphere::blue(Vec3::new(-0.3, 0.2, -3.0), 1.5));
    scene.add(Sphere::new(
        Vec3::new(0.5, -0.8, -7.0),
        4.0,
        Color::new(255., 0., 0.),
    ));

    scene
}

fn main() {
    let width_px = 640;
    let height_px = 640;

    let width_world: f64 = 2.0;
    let height_world: f64 = 2.0;

    let camera = Camera {
        eye: Point3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        vp_dir: Vec3::new(0.0, 0.0, -1.0),
    };

    let half_width_px = width_px as f64 / 2.0;
    let half_height_px = height_px as f64 / 2.0;

    let pixel_width = width_world / width_px as f64;
    let pixel_height = height_world / height_px as f64;

    let renderer = renderer::Renderer::new(width_px, height_px, build_scene());

    let vp_center = camera.eye.add(&camera.vp_dir);

    let mut img = RgbImage::new(width_px, height_px);
    for x_px in 0..width_px {
        let offset_x_px = (x_px as f64) - half_width_px;
        let offset_x_world = offset_x_px * pixel_width;
        for y_px in 0..height_px {
            let offset_y_px = (height_px - y_px) as f64 - half_height_px;
            let offset_y_world = offset_y_px * pixel_height;

            let world_offset = Point3::new(offset_x_world, offset_y_world, 0.0).add(&vp_center);
            let ray = Ray::from_points(camera.eye, world_offset);

            let color = renderer.calc_point(&ray);
            img.put_pixel(
                x_px,
                y_px,
                Rgb([color.x as u8, color.y as u8, color.z as u8]),
            );
        }
    }

    img.save("c:/delme/render3.png").unwrap();
}
