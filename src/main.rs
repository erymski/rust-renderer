use image::{Rgb, RgbImage};

mod geometry;

mod renderer;

use geometry::{Scene, Sphere, Vec3};

fn build_scene() -> Scene {
    let mut scene = Scene::new();
    scene.add(Sphere::new(Vec3::new(0.2, 0.8, 0.0), 5.0));

    scene
}

fn main() {
    let width = 128;
    let height = 128;

    let renderer = renderer::Renderer::new(width as usize, height as usize, build_scene());

    let mut img = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let color = renderer.calc_point(x, y);
            img.put_pixel(x, y, Rgb([color.x as u8, color.y as u8, color.z as u8]));
        }
    }

    img.save("c:/delme/render3.png").unwrap();
}
