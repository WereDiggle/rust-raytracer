extern crate raytracer;
extern crate euler;

use raytracer::*;
use euler::*;
use std::f64::consts::PI;
use std::path::Path;

fn light1() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(0.0, 300.0, 0.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn light2() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(-300.0, 300.0, 300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn light3() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(-300.0, -340.0, 300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn default_material(color: Color) -> Box<PhongShader> {
    PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0)
}

#[test]
fn transparent_shapes() {
    let scene = build_scene(
        vec!(light1(), light2(), light3()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: default_material(Color::GOLD),
                    floor: default_material(Color::BROWN),
                    front: default_material(Color::FIREBRICK),
                    back: default_material(Color::CYAN),
                    left: default_material(Color::ORANGE),
                    right: default_material(Color::LAVENDER),
                }),
                geometry_node(
                    translation(-150.0, 0.0, 0.0),
                    TranslucentShader::new(Color::WHITE, 1.52),
                    Sphere::from_radius(160.0),
                    vec!(),
                ),
                geometry_node(
                    translation(150.0, -220.0, 80.0),
                    TranslucentShader::new(Color::WHITE, 2.42),
                    Cube::new(160.0),
                    vec!(),
                ),
            ),
        ),
    );

    let image = render(scene, image(512, 512), camera([-300.0, 0.0, 300.0], [350.0, -350.0, -350.0]));
    write_to_png( image, "output/transparent_shapes");
}

#[test]
fn transparent_orb() {
    let scene = build_scene(
        vec!(light1(), light2(), light3()),
        no_ambient(),
        Some(SkyBox::from_path("assets/images/backgrounds/sky_ocean.jpg", rotation(Axis::Y, 20.0))),
        scene_node(
            DMat4::identity(),
            vec!(
                geometry_node(
                    translation(0.0, 0.0, 0.0),
                    TranslucentShader::new(Color::WHITE, 1.52),
                    Sphere::from_radius(160.0),
                    vec!(),
                ),
            ),
        ),
    );

    let image = render(scene, image(512, 512), camera([0.0, 0.0, 300.0], [0.0, 0.0, -0.0]));
    write_to_png( image, "output/transparent_shapes");
}