use image::{Rgb, RgbImage};

mod geometry;

mod renderer;

fn main() {
    let width = 128;
    let height = 128;

    let renderer = renderer::Renderer::new(width as usize, height as usize);

    let mut img = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let color = renderer.calc_point(x, y);
            img.put_pixel(x, y, Rgb([color.x as u8, color.y as u8, color.z as u8]));
        }
    }

    img.save("c:/delme/render2.png").unwrap();
}
