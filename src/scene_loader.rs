use crate::geometry::{
    AmbientLight, Camera, Color, DirectionalLight, Plane, Point3, RenderingSet, Scene, Sphere,
    UNIT_Y, UNIT_Z, Vec3, colors,
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

fn make_camera() -> Camera {
    let camera = Camera {
        eye: Point3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        vp_dir: Vec3::new(0.0, 0.0, -1.0),
    };
    camera
}

pub(crate) fn first_set() -> RenderingSet {
    RenderingSet {
        scene: build_scene(),
        camera: make_camera(),
    }
}

pub(crate) fn build_scene2() -> Scene {
    let mut scene = Scene::new();

    // lights
    scene.add_light(DirectionalLight::with_intensity(
        &Color::new(1.0, 0.88, 0.72),
        Vec3::new(0.55, -0.75, -0.35).normalize(),
        1.4,
    ));

    //// now ambient light is not necessary, because we have bouncing lighting
    // scene.ambient_light = Some(AmbientLight::with_intensity(
    //     &Color::new(0.12, 0.14, 0.18),
    //     1., // TODO: adjust ambient light intensity
    // ));

    // scene objects

    // large red sphere
    scene.add_object(Sphere::new(
        Point3::new(-1.35, 1.0, 0.2),
        1.0,
        Color::new(0.75, 0.08, 0.06),
    ));

    // small green sphere
    scene.add_object(Sphere::new(
        Point3::new(2.2, 0.35, 1.3),
        0.35,
        Color::new(0.20, 0.65, 0.08),
    ));

    // gray sphere (metallic in future)
    scene.add_object(Sphere::new(
        Point3::new(1.25, 0.65, -0.1),
        0.65,
        Color::new(0.85, 0.88, 0.92),
    ));

    // add ground plane
    scene.add_object(Plane::new(
        Point3::new(0.0, 0.0, 0.0), // cutting the red sphere
        &UNIT_Y,
        Color::new(0.72, 0.72, 0.70),
    ));

    scene
}

fn make_camera2() -> Camera {
    let from = Point3::new(0.0, 1.7, 5.5);
    let look_at = Vec3::new(0.0, 0.9, 0.0);
    let dir = (look_at - from).normalize();

    let camera = Camera {
        eye: from,
        up: UNIT_Y,
        vp_dir: dir,
    };
    camera
}

pub(crate) fn second_set() -> RenderingSet {
    RenderingSet {
        scene: build_scene2(),
        camera: make_camera2(),
    }
}
