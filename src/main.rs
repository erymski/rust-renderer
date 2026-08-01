use image::{Rgb, RgbImage};

mod vec2;
mod vec3;
// use vec3::{Color};

mod renderer;

fn main() {
    let width = 128;
    let height = 128;

    let renderer = renderer::Renderer::new(width as usize, height as usize);

    //    let pt = Vec3::new(1.0, 2.0, 3.0);

    let mut img = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let color = renderer.calc_point(x, y);
            img.put_pixel(x, y, Rgb([color.x as u8, color.y as u8, color.z as u8]));
        }
    }

    img.save("c:/delme/render.png").unwrap();

    //    println!("Hello, world! {} has length {}", pt.x, pt.length());
}
