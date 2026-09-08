use crate::geometry::{Color, DirectionalLight, Scene, Sphere, Vec3, colors};

fn build_directional_light() -> DirectionalLight {
    DirectionalLight::with_intensity(&colors::WHITE, Vec3::new(-1.0, 5.0, -1.0).normalize(), 1.0)
}

pub(crate) fn build_scene() -> Scene {
    let mut scene = Scene::new(build_directional_light());
    scene.add(Sphere::blue(Vec3::new(-0.3, 0.2, -3.0), 1.5));
    scene.add(Sphere::new(
        Vec3::new(0.5, -0.8, -7.0),
        4.0,
        Color::new(1., 0., 0.),
    ));

    scene
}
