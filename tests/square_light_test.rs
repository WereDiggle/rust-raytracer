extern crate raytracer;
extern crate euler;

use raytracer::*;
use euler::*;
use std::f64::consts::PI;
use std::path::Path;

fn square_light(pos: DVec3, size: f64) -> Box<SquareLight> {
    SquareLight::new(pos, size, Color::WHITE, 1500000.0, (0.0, 0.0, 4.0*PI))
}

fn light1() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(-100.0, 300.0, 300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 4.0*PI)))
}

fn light2() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(100.0, 300.0, -300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 4.0*PI)))
}

fn light3() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(-300.0, 300.0, 300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 4.0*PI)))
}

fn light4() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(0.0, -300.0, 0.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 4.0*PI)))
}

#[test]
fn square_light_basic() {
    let scene = build_scene(
        vec!(light2()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                geometry_node(
                    translation(0.0, 0.0, 0.0),
                    PhongShader::new(Color::RED*0.5, Color::RED*0.5, Color::BLACK, 4.0),
                    Sphere::from_radius(50.0),
                    vec!(),
                ),
                /*
                geometry_node(
                    translation(150.0, -270.0, 80.0),
                    PhongShader::new(Color::WHITE, Color::BLACK, Color::BLACK, 1.0),
                    Plane::new(dvec3!(0.0, 0.0, 0.0), dvec3!(0.0, 1.0, 0.0)),
                    vec!(),
                ),
                */
            ),
        ),
    );

    // config should be using a builder pattern
    let mut config = RenderConfig::default();
    config.anti_alias = false;
    let image = render_with_config(scene, image(256, 256), camera([0.0, 0.0, 100.0], [0.0, 0.0, 0.0]), config);
    write_to_png( image, "output/square_light_sphere");
}

#[test]
fn light_room() {
    let scene = build_scene(
        vec!(square_light(dvec3!(0.0, 340.0, 0.0), 500.0)),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: PhongShader::new(Color::WHITE, Color::BLACK, Color::BLACK, 1.0),
                    floor: PhongShader::new(Color::WHITE, Color::BLACK, Color::BLACK, 1.0),
                    front: PhongShader::new(Color::BLUE, Color::BLACK, Color::BLACK, 1.0),
                    back: PhongShader::new(Color::CYAN, Color::BLACK, Color::BLACK, 1.0),
                    left: PhongShader::new(Color::PURPLE, Color::BLACK, Color::BLACK, 1.0),
                    right: ReflectionShader::new(Color::WHITE),
                }),
                geometry_node(
                    translation(-150.0, -270.0, 0.0),
                    PhongShader::new(Color::WHITE*0.3, Color::WHITE*0.7, Color::BLACK, 4.0),
                    Sphere::from_radius(80.0),
                    vec!(),
                ),
                geometry_node(
                    translation(150.0, -270.0, 80.0),
                    texture_phong_material("assets/images/textures/cube_rgb_gradient.png", 0.5, 0.5, 0.0, 4.0),
                    Cube::new(160.0),
                    vec!(),
                ),
            ),
        ),
    );

    let image = render(scene, image(1000, 1000), camera([-300.0, 0.0, 300.0], [350.0, -350.0, -350.0]));
    write_to_png( image, "output/light_room");
}