use image::{Rgb, RgbImage};

mod geometry;

mod renderer;

use geometry::{Circle, Color, Rect2d, Scene2d, Vec2};

fn build_scene() -> Scene2d {
    let mut scene = Scene2d::new();
    scene.add_object(Rect2d::new(
        Vec2::new(0.2, 0.8),
        Vec2::new(0.2, 0.8),
        Color::new(128., 54., 204.),
    ));
    scene.add_object(Circle::new(
        Vec2::new(0.1, 0.3),
        0.25,
        Color::new(255., 0., 0.),
    ));
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
