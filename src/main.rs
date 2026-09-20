use image::{Rgb, RgbImage};
use std::time::Instant;

mod geometry;
mod renderer;
mod scene_loader;

use geometry::{Color, Point3, Ray, RenderingSet, ToneMapper, color_clamp};

use crate::scene_loader::second_set;

#[allow(clippy::cast_possible_truncation)]
fn to_rgb(color: &Color) -> Rgb<u8> {
    Rgb([
        (color.x * 255.) as u8,
        (color.y * 255.) as u8,
        (color.z * 255.) as u8,
    ])
}

fn main() {
    let tone_mapper: ToneMapper = color_clamp; //;color_reinhard; //color_identity;

    let width_px = 1024;
    let height_px = 768;

    let width_world: f64 = 4.0;
    let height_world: f64 = 3.0;

    let RenderingSet { scene, camera } = second_set();

    let half_width_px = f64::from(width_px) / 2.0;
    let half_height_px = f64::from(height_px) / 2.0;

    let pixel_width = width_world / f64::from(width_px);
    let pixel_height = height_world / f64::from(height_px);

    let renderer = renderer::Renderer::new(width_px, height_px, scene);

    let vp_center = camera.eye + camera.vp_dir;

    let mut img = RgbImage::new(width_px, height_px);

    println!("Start rendering");
    let start = Instant::now();

    for x_px in 0..width_px {
        let offset_x_px = f64::from(x_px) - half_width_px;
        let offset_x_world = offset_x_px * pixel_width;
        for y_px in 0..height_px {
            let offset_y_px = half_height_px - f64::from(y_px);
            let offset_y_world = offset_y_px * pixel_height;

            // TODO: will not work with arbitrary camera orientation
            let pixel_in_world = Point3::new(offset_x_world, offset_y_world, 0.0) + vp_center;
            let ray = Ray::from_points(camera.eye, &pixel_in_world);

            let color = renderer.calc_point(&ray);
            let squeezed_color = tone_mapper(&color);

            img.put_pixel(x_px, y_px, to_rgb(&squeezed_color));
        }
    }

    println!("Rendering took {:.3} sec", start.elapsed().as_secs_f64());

    img.save("c:/delme/render3.png").unwrap();
}
