extern crate raytracer;
extern crate euler;

use raytracer::*;
use raytracer::matrix::*;
use euler::*;
use std::f64::consts::PI;

fn light1() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(0.0, 100.0, 0.0), Color::new(1.0, 1.0, 1.0), 90000.0, (0.0, 0.0, 4.0*PI)))
}

fn light2() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(-50.0, 100.0, 100.0), Color::new(1.0, 1.0, 1.0), 75000.0, (0.0, 0.0, 4.0*PI)))
}

fn light3() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(50.0, 100.0, -100.0), Color::new(1.0, 1.0, 1.0), 75000.0, (0.0, 0.0, 4.0*PI)))
}

fn front_light() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(0.0, 0.0, 100.0), Color::new(1.0, 1.0, 1.0), 1.0, (1.0, 0.0, 0.0)))
}

fn wood_material() -> Box<CompositeShader> {
    CompositeShader::from_shaders(
        vec!(
            (0.8, MixShader::from_shaders(
                vec!(
                    PhongShader::new(Color::WHITE*0.5, Color::WHITE*0.5, Color::BLACK, 2.0),
                    TextureShader::new(ImageTexture::from_path("assets/images/textures/light_wood.jpg")),            
                ),
            )),
            (0.2, ReflectionShader::new(Color::WHITE)),
        )
    )
}

fn make_character_sheet(size: f64, transform: DMat4) -> Box<SceneNode> {
    geometry_node(
        transform*translation(0.0, 0.01, 0.0)*rotation(Axis::X, -90.0),
        texture_phong_material("assets/images/textures/gurf-1.jpg", 1.0, 0.0, 0.0, 1.0),
        Rectangle::new(1.7*size, 2.2*size),
        vec!(),
    )
}

fn make_d6(size: f64, transform: DMat4) -> Box<SceneNode> {
    geometry_node(
        transform*translation(0.0, size/2.0, 0.0),
        ChainShader::from_shaders(vec!(
            NormalMapShader::new(BumpMap::from_path("assets/images/bump_maps/d6_num.png", 10.0)),
            CompositeShader::from_shaders(vec!(
                (0.05, ReflectionShader::new(Color::WHITE)),
                (0.95, texture_phong_material("assets/images/textures/d6_num.png", 0.9, 0.1, 0.0, 2.0)),
            )),
        )),
        Cube::new(size),
        vec!()
    )
}

fn make_d4(size: f64, transform: DMat4) -> Box<SceneNode> {
    geometry_node(
        transform,
        texture_phong_material("assets/images/textures/d4_num.png", 1.0, 0.0, 0.0, 1.0),
        Tetrahedron::new(size),
        vec!()
    )
}

fn make_dice_scene() -> Scene {
    build_scene(
        vec!(light1(), light2()),
        no_ambient(),
        Some(SkyBox::from_path("assets/images/backgrounds/nice_room.jpg", rotation(Axis::Y, 20.0))),
        scene_node(
            DMat4::identity(),
            vec!(
                create_floor_from_material(200.0, wood_material()),
                make_character_sheet(30.0, translation(-20.0, 0.0, 50.0)*rotation(Axis::Y, 230.0)),
                make_d6(5.0, translation(-30.0, 0.0, 50.0)),
                make_d4(5.0, translation(-20.0, 0.0, 50.0)),
            ),
        ),
    )
}

fn make_camera() -> CameraConfig {
    camera([-30.0, 20.0, 70.0], [50.0, -50.0, -80.0])
}

#[test]
fn dice_scene_lo_res() {
    let scene = make_dice_scene();
    let image = render(scene, image(192, 108), camera([-30.0, 20.0, 70.0], [50.0, -50.0, -80.0]));
    write_to_png( image, "output/dice_scene_lo_res");
}

#[test]
fn dice_scene_hi_res() {
    let scene = make_dice_scene();
    let mut config = RenderConfig::default();
    config.anti_alias = false;
    let image = render_with_config(scene, image(1920, 1080), camera([-30.0, 20.0, 70.0], [50.0, -50.0, -80.0]), config);
    write_to_png( image, "output/dice_scene_hi_res");
}

#[test]
fn triangle_basic() {
    let scene = build_scene(
        vec!(front_light()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                geometry_node(
                    rotation(Axis::Y, 0.0),
                    texture_phong_material("assets/images/textures/granite.jpg", 1.0, 0.0, 0.0, 1.0),
                    Triangle::from_vertices(dvec3!(-10.0, -10.0, 0.0), dvec3!(0.0, -10.0, 0.0), dvec3!(10.0, 10.0, 0.0)),
                    vec!()
                ),
            ),
        ),
    );
    let image = render(scene, image(512, 512), camera([0.0, 0.0, 10.0], [0.0, 0.0, -10.0]));
    write_to_png( image, "output/triangle");
}

#[test]
fn tetrahedron_basic() {
    let scene = build_scene(
        vec!(light1(), light2(), light3()),
        no_ambient(),
        None,
        scene_node(
            rotation(Axis::X, -50.0),
            vec!(
                make_d4(30.0, translation(0.0, 0.0, 0.0)),
            ),
        ),
    );
    let image = render(scene, image(512, 512), camera([0.0, 20.0, 40.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/tetrahedron");
}