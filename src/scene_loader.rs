use crate::geometry::{
    C, Camera, DirectionalLight, Lambertian, Material, P, Plane, RenderingSet, Scene, Sphere,
    UNIT_Y, V,
};

// pub(crate) fn build_scene() -> Scene {
//     let mut scene = Scene::new();

//     // lights
//     scene.add_light(DirectionalLight::with_intensity(
//         &colors::WHITE,
//         V(-1.0, 5.0, -1.0).normalize(),
//         1.0,
//     ));

//     scene.add_light(DirectionalLight::with_intensity(
//         &colors::WHITE,
//         V(5.0, -2.0, -1.0).normalize(),
//         1.0,
//     ));

//     scene.ambient_light = Some(AmbientLight::with_intensity(&colors::WHITE, 0.3));

//     // scene objects
//     scene.add_object(Sphere::blue(P(-0.3, 0.2, -3.0), 1.5));
//     scene.add_object(Sphere::new(P(0.5, -0.8, -7.0), 4.0, C(1., 0., 0.)));

//     // add ground
//     scene.add_object(Plane::new(
//         P(4., 5., -3.3), // cutting the red sphere
//         &UNIT_Z,
//         colors::GREEN,
//     ));

//     scene
// }

// fn make_camera() -> Camera {
//     let camera = Camera {
//         eye: P(0.0, 0.0, 0.0),
//         up: V(0.0, 1.0, 0.0),
//         vp_dir: V(0.0, 0.0, -1.0),
//     };
//     camera
// }

// pub(crate) fn first_set() -> RenderingSet {
//     RenderingSet {
//         scene: build_scene(),
//         camera: make_camera(),
//     }
// }

pub(crate) fn build_scene2() -> Scene {
    let mut scene = Scene::new();

    // materials
    let red_index = scene.add_material(Material::Lambertian(Lambertian {
        albedo: C(0.75, 0.08, 0.06),
    }));

    let green_index = scene.add_material(Material::Lambertian(Lambertian {
        albedo: C(0.20, 0.65, 0.08),
    }));

    let gray_index = scene.add_material(Material::Lambertian(Lambertian {
        albedo: C(0.85, 0.88, 0.92),
    }));

    let ground_index = scene.add_material(Material::Lambertian(Lambertian {
        albedo: C(0.72, 0.72, 0.70),
    }));

    // lights
    scene.add_light(DirectionalLight::with_intensity(
        &C(1.0, 0.88, 0.72),
        V(0.55, -0.75, -0.35).normalize(),
        1.4,
    ));

    //// now ambient light is not necessary, because we have bouncing lighting
    // scene.ambient_light = Some(AmbientLight::with_intensity(
    //     &C(0.12, 0.14, 0.18),
    //     1., // TODO: adjust ambient light intensity
    // ));

    // scene objects

    // large red sphere
    scene.add_object(Sphere::new(P(-1.35, 1.0, 0.2), 1.0, red_index));

    // small green sphere
    scene.add_object(Sphere::new(P(2.2, 0.35, 1.3), 0.35, green_index));

    // gray sphere (metallic in future)
    scene.add_object(Sphere::new(P(1.25, 0.65, -0.1), 0.65, gray_index));

    // add ground plane
    scene.add_object(Plane::new(
        P(0.0, 0.0, 0.0), // cutting the red sphere
        &UNIT_Y,
        ground_index,
    ));

    scene
}

fn make_camera2() -> Camera {
    let from = P(0.0, 1.7, 5.5);
    let look_at = V(0.0, 0.9, 0.0);
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
