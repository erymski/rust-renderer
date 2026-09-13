use crate::geometry::{
    AmbientLight, Color, DirectionalLight, Plane, Point3, Scene, Sphere, UNIT_X, UNIT_Y, UNIT_Z,
    Vec3, colors,
};

pub(crate) fn build_scene() -> Scene {
    let mut scene = Scene::new();

    // lights
    scene.add_light(DirectionalLight::with_intensity(
        &colors::WHITE,
        Vec3::new(-1.0, 5.0, -1.0).normalize(),
        1.0,
    ));

    scene.add_light(DirectionalLight::with_intensity(
        &colors::WHITE,
        Vec3::new(5.0, -2.0, -1.0).normalize(),
        1.0,
    ));

    scene.ambient_light = Some(AmbientLight::with_intensity(&colors::WHITE, 0.3));

    // scene objects
    scene.add_object(Sphere::blue(Point3::new(-0.3, 0.2, -3.0), 1.5));
    scene.add_object(Sphere::new(
        Point3::new(0.5, -0.8, -7.0),
        4.0,
        Color::new(1., 0., 0.),
    ));

    // add ground
    scene.add_object(Plane::new(
        Point3::new(4., 5., -3.3), // cutting the red sphere
        &UNIT_Z,
        colors::GREEN,
    ));

    scene
}
