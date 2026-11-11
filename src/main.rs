use image::{Rgb, RgbImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::time::Instant;

mod geometry;
mod renderer;
mod scene_loader;
mod utils;

use geometry::{Color, P, Ray, RenderingSet, ToneMapper, color_clamp};

use crate::{
    geometry::RenderingContext,
    scene_loader::second_set,
    utils::{ExecutionParams, ImageParams, get_params},
};

#[allow(clippy::cast_possible_truncation)]
fn to_rgb(color: &Color) -> Rgb<u8> {
    Rgb([
        (color.x * 255.) as u8,
        (color.y * 255.) as u8,
        (color.z * 255.) as u8,
    ])
}

fn main() {
    let params = get_params();

    let tone_mapper: ToneMapper = color_clamp; //;color_reinhard; //color_identity;

    let RenderingSet { scene, camera } = second_set();

    run(params, tone_mapper, scene, camera);
}

fn run(
    params: ExecutionParams,
    tone_mapper: fn(&geometry::Vec3) -> geometry::Vec3,
    scene: geometry::Scene,
    camera: geometry::Camera,
) {
    let width_world: f64 = 4.0; // TODO:
    let height_world: f64 = 3.0; // TODO:

    let ImageParams {
        width_px,
        height_px,
    } = params.rendering;

    let half_width_px = f64::from(width_px) / 2.0;
    let half_height_px = f64::from(height_px) / 2.0;

    let pixel_width = width_world / f64::from(width_px);
    let pixel_height = height_world / f64::from(height_px);

    let renderer = renderer::Renderer::new(width_px, height_px, scene);

    let vp_center = camera.eye + camera.vp_dir;

    let img = std::sync::Mutex::new(RgbImage::new(width_px, height_px));

    println!("Start rendering");
    let start = Instant::now();

    (0..width_px * height_px).into_par_iter().for_each(|i| {
        let x_px = i % width_px;
        let y_px = i / width_px;

        let offset_x_px = f64::from(x_px) - half_width_px;
        let offset_x_world = offset_x_px * pixel_width;

        let offset_y_px = half_height_px - f64::from(y_px);
        let offset_y_world = offset_y_px * pixel_height;

        // TODO: will not work with arbitrary camera orientation
        let pixel_in_world = P(offset_x_world, offset_y_world, 0.0) + vp_center;
        let ray = Ray::from_points(camera.eye, &pixel_in_world);

        // let color = renderer.calc_point(&ray);
        let color = renderer.go(
            &ray,
            &RenderingContext {
                x: x_px,
                y: y_px,
                width: width_px,
                height: height_px,
            },
        );
        let squeezed_color = tone_mapper(&color);

        img.lock()
            .unwrap()
            .put_pixel(x_px, y_px, to_rgb(&squeezed_color));
    });

    println!("Rendering took {:.3} sec", start.elapsed().as_secs_f64());

    img.lock().unwrap().save(params.output_file).unwrap();
}
