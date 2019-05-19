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

fn square_light(pos: DVec3, size: f64) -> Box<SquareLight> {
    SquareLight::new(pos, size, Color::WHITE, 1500000.0, (0.0, 0.0, 4.0*PI))
}

fn default_material(color: Color) -> Box<PhongShader> {
    PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0)
}

fn glass() -> Box<CompositeShader> {
    CompositeShader::from_shaders(vec!(
        (0.1, ReflectionShader::new(Color::WHITE)),
        (0.9, TranslucentShader::new(Color::WHITE, 1.517)),
    ))
}

fn frosted_glass() -> Box<ChainShader> {
    ChainShader::from_shaders(vec!(
        NormalMapShader::new(NormalMap::from_path("assets/images/normal_maps/concrete.jpg")),
        CompositeShader::from_shaders(vec!(
            (0.1, ReflectionShader::new(Color::WHITE)),
            (0.9, TranslucentShader::new(Color::WHITE, 1.517)),
        )),
    ))
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

#[test]
fn glass_monkey() {
    let scene = build_scene(
        vec!(light1(), light2(), light3()),
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
                    glass(),
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
    config.anti_alias = true;
    let image = render_with_config(scene, image(1000, 1000), camera([-310.0, 200.0, 300.0], [0.0, 0.0, 0.0]), config);
    write_to_png( image, "output/glass_monkey");
}

fn ceramic_tile() -> Box<ChainShader> {
    ChainShader::from_shaders(vec!(
        NormalMapShader::new(BumpMap::from_path("assets/images/bump_maps/tiles.jpg", 10.0)),
        CompositeShader::from_shaders(vec!(
            (0.8, MixShader::from_shaders(vec!(
                    PhongShader::new(Color::WHITE*0.5, Color::WHITE*0.5, Color::BLACK, 2.0),
                    TextureShader::new(ImageTexture::from_path("assets/images/textures/marble.png")),
                  ))
            ),
            (0.2, ReflectionShader::new(Color::WHITE))
        )),
    ))    
}

fn concrete(color: Color) -> Box<ChainShader> {
    ChainShader::from_shaders(vec!(
        NormalMapShader::new(NormalMap::from_path("assets/images/normal_maps/concrete.jpg")),
        MixShader::from_shaders(vec!(
            PhongShader::new(color*1.0, Color::WHITE*0.0, Color::BLACK, 1.0),
            TextureShader::new(ImageTexture::from_path("assets/images/textures/smooth_concrete.jpg")),
        ))
    ))
}

fn metallic() -> Box<ChainShader> {
    ChainShader::from_shaders(vec!(
        NormalMapShader::new(NormalMap::from_path("assets/images/normal_maps/concrete.jpg")),
        CompositeShader::from_shaders(vec!(
            (0.3, ReflectionShader::new(Color::WHITE)),
            (0.7, PhongShader::new(Color::WHITE*0.2, Color::BLACK, Color::BLACK, 2.0)),
        )),
    ))
}

#[test]
fn monkey_1() {
    let scene = build_scene(
        vec!(square_light(dvec3!(0.0, 340.0, 0.0), 500.0)),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: default_material(Color::WHITE),
                    floor: ceramic_tile(),
                    front: default_material(Color::MAGENTA),
                    back: default_material(Color::CYAN),
                    left: default_material(Color::YELLOW),
                    right: default_material(Color::GREEN),
                }),
                geometry_node(
                    translation(0.0, 0.0, 0.0)*scaling(150.0, 150.0, 150.0)*rotation(Axis::Y, -30.0),
                    concrete(Color::WHITE),
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

    let image = render(scene, image(1000, 1000), camera([-310.0, 200.0, 300.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/monkey_1");
}

#[test]
fn monkey_2() {
    let scene = build_scene(
        vec!(square_light(dvec3!(0.0, 340.0, 0.0), 200.0)),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: default_material(Color::WHITE),
                    floor: ceramic_tile(),
                    front: concrete(Color::FIREBRICK),
                    back: default_material(Color::CYAN),
                    left: default_material(Color::MAGENTA),
                    right: metallic(),
                }),
                geometry_node(
                    translation(0.0, 0.0, 0.0)*scaling(150.0, 150.0, 150.0)*rotation(Axis::Y, -30.0),
                    metallic(),
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

    let image = render(scene, image(1920, 1920), camera([-310.0, 200.0, 300.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/monkey_2");
}

#[test]
fn monkey_3() {
    let scene = build_scene(
        vec!(square_light(dvec3!(0.0, 340.0, 0.0), 200.0)),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: default_material(Color::WHITE),
                    floor: ceramic_tile(),
                    front: concrete(Color::FIREBRICK),
                    back: default_material(Color::CYAN),
                    left: default_material(Color::MAGENTA),
                    right: metallic(),
                }),
                geometry_node(
                    translation(0.0, 0.0, 0.0)*scaling(150.0, 150.0, 150.0)*rotation(Axis::Y, -30.0),
                    frosted_glass(),
                    Mesh::from_path(&Path::new("assets/models/monkey2.obj")),
                    vec!(),
                ),
            ),
        ),
    );

    let image = render(scene, image(1920, 1920), camera([-310.0, 200.0, 300.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/monkey_3");
}
