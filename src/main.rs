extern crate raytracer;
extern crate euler;

use raytracer::*;
use euler::*;
use std::f64::consts::PI;
use std::path::Path;

fn light1() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(-100.0, 300.0, 300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn light2() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(100.0, 300.0, -300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn light3() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(-300.0, 300.0, 300.0), Color::new(1.0, 1.0, 1.0), 450000.0, (0.0, 0.0, 1.0*PI)))
}

fn light4() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(0.0, -300.0, 0.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn front_light() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(0.0, 0.0, 1000.0), Color::new(1.0, 1.0, 1.0), 1.0, (1.0, 0.0, 0.0)))
}

fn square_light(pos: DVec3, size: f64) -> Box<SquareLight> {
    SquareLight::new(pos, size, Color::WHITE, 1500000.0, (0.0, 0.0, 4.0*PI))
}

fn default_material(color: Color) -> Box<PhongShader> {
    PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0)
}

fn mesh_basic() {
    let scene = build_scene(
        vec!(light3()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: default_material(Color::RED),
                    floor: default_material(Color::BLUE),
                    front: default_material(Color::MAGENTA),
                    back: default_material(Color::CYAN),
                    left: default_material(Color::YELLOW),
                    right: default_material(Color::GREEN),
                }),
                geometry_node(
                    translation(0.0, 0.0, 0.0)*scaling(200.0, 200.0, 200.0)*rotation(Axis::Y, -30.0),
                    texture_phong_material("assets/images/textures/test3.png", 1.0, 0.0, 0.0, 1.0),
                    Mesh::from_path(&Path::new("assets/models/monkey2.obj")),
                    vec!(),
                ),
                /*
                geometry_node(
                    translation(0.0, 0.0, 0.0)*scaling(100.0, 100.0, 100.0),
                    default_material(Color::RED),
                    Cube::new(4.0),
                    vec!(),
                ),
                */
            ),
        ),
    );

    let mut config = RenderConfig::default();
    config.anti_alias = false;
    let image = render_with_config(scene, image(256, 256), camera([-310.0, 200.0, 300.0], [0.0, 0.0, 0.0]), config);
    write_to_png( image, "output/mesh_basic");
}

fn main() {
    mesh_basic();
}