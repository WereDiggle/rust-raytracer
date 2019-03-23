extern crate raytracer;
extern crate euler;

use raytracer::*;
use euler::*;
use std::f64::consts::PI;

fn light1() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(200.0, 200.0, -200.0), Color::new(1.0, 1.0, 1.0), 300000.0, (0.0, 0.0, 1.0*PI)))
}

fn light2() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(200.0, 200.0, 0.0), Color::new(1.0, 1.0, 1.0), 300000.0, (0.0, 0.0, 1.0*PI)))
}

pub fn test_bump_shader(color: Color) -> Box<ChainShader> {
    let mut bump_shader = ChainShader::new();
    bump_shader.push_shader(
        NormalMapShader::new(
            BumpMap::from_path("assets/images/bump_maps/dot.png", 1.0)
        )
    );
    bump_shader.push_shader(
        PhongShader::new(color*1.0, Color::BLACK, Color::BLACK, 1.0)
    );
    bump_shader
}

fn bump_map_basic_1() {
    let scene = build_scene(
        vec!(light1()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: test_bump_shader(Color::WHITE),
                    floor: test_bump_shader(Color::WHITE),
                    front: test_bump_shader(Color::WHITE),
                    back: test_bump_shader(Color::WHITE),
                    left: test_bump_shader(Color::WHITE),
                    right: test_bump_shader(Color::WHITE),
                }),
            ),
        ),
    );

    let image = render(scene, image(500, 500), camera([-300.0, 0.0, 300.0], [350.0, -350.0, -350.0]));
    write_to_png( image, "output/basic_bump_room_1");
}

fn bump_map_basic_2() {
    let scene = build_scene(
        vec!(light2()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: test_bump_shader(Color::WHITE),
                    floor: test_bump_shader(Color::WHITE),
                    front: test_bump_shader(Color::WHITE),
                    back: test_bump_shader(Color::WHITE),
                    left: test_bump_shader(Color::WHITE),
                    right: test_bump_shader(Color::WHITE),
                }),
            ),
        ),
    );

    let image = render(scene, image(500, 500), camera([-300.0, 0.0, 300.0], [350.0, -350.0, -350.0]));
    write_to_png( image, "output/basic_bump_room_2");
}


fn main() {
    bump_map_basic_1();
}
