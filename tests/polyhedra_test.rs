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

fn light3() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(-300.0, 300.0, 300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn light4() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(0.0, -300.0, 0.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn default_material(color: Color) -> Box<PhongShader> {
    PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0)
}

fn triangular_prism() -> Box<Polyhedron> {
    Polyhedron::from_planes(vec!(
        *Plane::new(dvec3!(0.0, -1.0, 0.0), dvec3!(0.0, -1.0, 0.0)),     // bottom
        *Plane::new(dvec3!(1.0, 0.0, 0.0), dvec3!(1.0, 0.0, 0.0)),       // right
        *Plane::new(dvec3!(-1.0, 0.0, 0.0), dvec3!(-1.0, 0.0, 0.0)),     // left
        *Plane::new(dvec3!(0.0, -1.0, -1.0), dvec3!(0.0, 1.0, -1.0)),    // back
        *Plane::new(dvec3!(0.0, -1.0, 1.0), dvec3!(0.0, 1.0, 1.0)),     // front
    ))
}

#[test]
fn polyhedron_basic() {
    let scene = build_scene(
        vec!(light1(), light2(), light3()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: default_material(Color::WHITE),
                    floor: default_material(Color::WHITE),
                    front: default_material(Color::RED),
                    back: default_material(Color::CYAN),
                    left: default_material(Color::GREEN),
                    right: ReflectionShader::new(Color::WHITE),
                }),
                geometry_node(
                    scaling(50.0, 200.0, 100.0)*rotation(Axis::Y, -45.0),
                    texture_phong_material("assets/images/textures/granite.jpg", 0.5, 0.5, 0.0, 2.0),
                    triangular_prism(),
                    vec!(),
                ),
            ),
        ),
    );

    let image = render(scene, image(512, 512), camera([-300.0, 0.0, 300.0], [350.0, -350.0, -350.0]));
    write_to_png( image, "output/polyhedron_basic");
}

#[test]
fn octahedron_basic() {
    let scene = build_scene(
        vec!(light1(), light2(), light3()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: default_material(Color::WHITE),
                    floor: default_material(Color::WHITE),
                    front: default_material(Color::RED),
                    back: default_material(Color::CYAN),
                    left: default_material(Color::GREEN),
                    right: ReflectionShader::new(Color::WHITE),
                }),
                geometry_node(
                    translation(0.0, 0.0, 0.0),
                    texture_phong_material("assets/images/textures/d8_num.png", 1.0, 0.0, 0.0, 2.0),
                    Polyhedron::octahedron(200.0),
                    vec!(),
                ),
            ),
        ),
    );

    let img = render(scene.clone(), image(512, 512), camera([-300.0, 0.0, 300.0], [0.0, 0.0, 0.0]));
    write_to_png( img, "output/octahedron_basic_01");

    let img = render(scene.clone(), image(512, 512), camera([300.0, 0.0, -300.0], [0.0, 0.0, 0.0]));
    write_to_png( img, "output/octahedron_basic_02");
}
