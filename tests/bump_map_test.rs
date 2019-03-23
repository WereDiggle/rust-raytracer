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

pub fn test_bump_shader(color: Color, normal_map: Box<NormalMappable + Send + Sync>) -> Box<ChainShader> {
    let mut bump_shader = ChainShader::new();
    bump_shader.push_shader(
        NormalMapShader::new(
            normal_map,
        )
    );
    bump_shader.push_shader(
        PhongShader::new(color*1.0, Color::BLACK, Color::BLACK, 1.0)
    );
    bump_shader
}

#[test]
fn bump_map_basic_1() {
    let mut am = AssetManager::new();
    let scene = build_scene(
        vec!(light1()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 3.0)),
                    floor: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 1.0)),
                    front: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 1.0)),
                    back: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 1.0)),
                    left: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 1.0)),
                    right: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 1.0)),
                }),
            ),
        ),
    );

    let image = render(scene, image(500, 500), camera([-300.0, 0.0, 300.0], [350.0, -350.0, -350.0]));
    write_to_png( image, "output/basic_bump_room_1");
}

#[test]
fn bump_map_basic_2() {
    let mut am = AssetManager::new();
    let scene = build_scene(
        vec!(light2()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 3.0)),
                    floor: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 1.0)),
                    front: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 1.0)),
                    back: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 1.0)),
                    left: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 1.0)),
                    right: test_bump_shader(Color::WHITE, am.bump_map_from_path("assets/images/bump_maps/bumps.png", 1.0)),
                }),
            ),
        ),
    );

    let image = render(scene, image(500, 500), camera([-300.0, 0.0, 300.0], [350.0, -350.0, -350.0]));
    write_to_png( image, "output/basic_bump_room_2");
}
