extern crate raytracer;
extern crate euler;

use raytracer::*;
use raytracer::matrix::*;
use euler::*;
use std::f64::consts::PI;

fn light1() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(0.0, 100.0, 0.0), Color::new(1.0, 1.0, 1.0), 100000.0, (0.0, 0.0, 1.0*PI)))
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

fn make_dice_scene() -> Scene {
    build_scene(
        vec!(light1()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_floor_from_material(200.0, wood_material()),
            ),
        ),
    )
}

#[test]
fn dice_scene() {
    let scene = make_dice_scene();
    let image = render(scene, image(512, 512), camera([-50.0, 50.0, 100.0], [50.0, 0.0, -80.0]));
    write_to_png( image, "output/dice_scene");
}