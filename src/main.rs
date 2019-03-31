extern crate raytracer;
extern crate euler;

use raytracer::*;
use euler::*;
use std::f64::consts::PI;

fn light1() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(0.0, 100.0, 0.0), Color::new(1.0, 1.0, 1.0), 75000.0, (0.0, 0.0, 1.0*PI)))
}
fn light2() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(-50.0, 100.0, 100.0), Color::new(1.0, 1.0, 1.0), 75000.0, (0.0, 0.0, 1.0*PI)))
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
        CompositeShader::from_shaders(vec!(
            //(0.05, ReflectionShader::new(Color::WHITE)),
            (1.0, ChainShader::from_shaders(vec!(
                NormalMapShader::new(BumpMap::from_path("assets/images/bump_maps/d6_num.png", 3.0)),
                texture_phong_material("assets/images/textures/d6_num.png", 0.9, 0.1, 0.0, 2.0),
            )))
        )),
        Cube::new(size),
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
                make_d6(10.0, translation(-30.0, 0.0, 50.0)),
            ),
        ),
    )
}

fn dice_scene() {
    let scene = make_dice_scene();
    let image = render(scene, image(512, 512), camera([-50.0, 20.0, 100.0], [50.0, -50.0, -80.0]));
    write_to_png( image, "output/dice_scene");
}

fn main() {
    dice_scene();
}