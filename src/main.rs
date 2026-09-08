use image::{Rgb, RgbImage};

mod geometry;
mod renderer;
mod scene_loader;

use geometry::{Camera, Color, Point3, Ray, ToneMapper, Vec3, color_identity};

use crate::scene_loader::build_scene;

fn to_rgb(color: &Color) -> Rgb<u8> {
    Rgb([
        (color.x * 255.) as u8,
        (color.y * 255.) as u8,
        (color.z * 255.) as u8,
    ])
}

fn main() {
    let tone_mapper: ToneMapper = color_identity;

    let width_px = 1024;
    let height_px = 768;

    let width_world: f64 = 4.0;
    let height_world: f64 = 3.0;

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
            let offset_y_px = half_height_px - y_px as f64;
            let offset_y_world = offset_y_px * pixel_height;

            // TODO: will not work with arbitrary camera orientation
            let pixel_in_world = Point3::new(offset_x_world, offset_y_world, 0.0).add(&vp_center);
            let ray = Ray::from_points(camera.eye, &pixel_in_world);

            let color = renderer.calc_point(&ray);
            let squeezed_color = tone_mapper(&color);
            img.put_pixel(x_px, y_px, to_rgb(&squeezed_color));
        }
    }

    img.save("c:/delme/render3.png").unwrap();
}
