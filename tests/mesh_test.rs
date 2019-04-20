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
    Box::new(PointLight::new(dvec3!(-300.0, 300.0, 300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn light4() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(0.0, -300.0, 0.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn front_light() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(0.0, 0.0, 1000.0), Color::new(1.0, 1.0, 1.0), 1.0, (1.0, 0.0, 0.0)))
}

fn default_material(color: Color) -> Box<PhongShader> {
    PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0)
}

#[test]
fn mesh_basic() {
    let scene = build_scene(
        vec!(light1(), light2(), light3(), light4()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: default_material(Color::WHITE),
                    floor: default_material(Color::WHITE),
                    front: default_material(Color::WHITE),
                    back: default_material(Color::CYAN),
                    left: default_material(Color::YELLOW),
                    right: default_material(Color::WHITE),
                }),
                geometry_node(
                    translation(0.0, 0.0, 0.0)*scaling(100.0, 100.0, 100.0),
                    default_material(Color::RED),
                    Mesh::from_path(&Path::new("assets/models/my_teapot2.obj")),
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

    let image = render(scene, image(512, 512), camera([-310.0, 300.0, 300.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/mesh_basic");
}
