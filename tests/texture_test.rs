extern crate raytracer;
extern crate euler;

use raytracer::*;
use euler::*;
use std::f64::consts::PI;

fn light1() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(-100.0, 300.0, 300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn light2() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(100.0, 300.0, -300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

#[test]
fn sphere_texture() {
    let sphere_size: f64 = 80.0;
    let scene = build_scene(
        vec!(light1(), light2()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room(700.0, RoomColorScheme {
                    ceiling: Color::RED,
                    floor: Color::GREEN,
                    front: Color::BLUE,
                    back: Color::CYAN,
                    left: Color::MAGENTA,
                    right: Color::YELLOW,
                }),
                geometry_node(
                    translation(0.0, 0.0, 40.0),
                    texture_phong_material("assets/images/textures/denim.jpg", 1.0, 0.0, 0.0, 1.0),
                    Box::new(Sphere::new(sphere_size)),
                    vec!(),
                ),
            ),
        ),
    );

    let image = render(scene, image(1920, 1080), camera([0.0, 0.0, 350.0], [0.0, -100.0, -350.0]));
    write_to_png( image, "output/texture_sphere");
}