extern crate raytracer;
extern crate euler;

use raytracer::*;
use euler::*;
use std::f64::consts::PI;

fn light1() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(200.0, 200.0, -200.0), Color::new(1.0, 1.0, 1.0), 300000.0, (0.0, 0.0, 1.0*PI)))
}

fn light2() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(100.0, 300.0, -300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn no_ambient() -> AmbientLight {
    AmbientLight::new(Color::WHITE, 0.0)
}

fn clear_material() -> Box<TranslucentShader> {
    TranslucentShader::new(Color::WHITE, 1.52)
}

fn diffuse_material() -> Box<PhongShader> {
    PhongShader::new(Color::WHITE*1.0, Color::WHITE*0.0, Color::WHITE*0.0, 1.0)
}

fn make_half_sphere(size: f64) -> Box<SubtractShape> {
    subtract_shape(
        DMat4::identity(),
        base_shape(
            translation(0.0, 0.0, 0.0),
            sphere(size),
        ),
        base_shape(
            translation(size, 0.0, 0.0),
            sphere(size),
        ),
    )
}
fn make_spikey_thing(size: f64) -> Box<SubtractShape> {
    subtract_shape(
        DMat4::identity(),
        subtract_shape(
            DMat4::identity(),
            subtract_shape(
                DMat4::identity(),
                base_shape(
                    DMat4::identity(),
                    sphere(size),
                ),
                base_shape(
                    translation(size, 0.0, 0.0),
                    sphere(size),
                ),
            ),
            base_shape(
                translation(0.0, size, 0.0),
                sphere(size),
            ),
        ),
        base_shape(
            translation(0.0, 0.0, size),
            sphere(size),
        ),
    )
}

fn make_weird_cube(size: f64) -> Box<SubtractShape> {
    subtract_shape(
        DMat4::identity(),
        base_shape(
            DMat4::identity(),
            cube(size*2.0),
        ),
        base_shape(
            DMat4::identity(),
            sphere(size),
        ),
    )
}

fn default_material(color: Color) -> Box<PhongShader> {
    PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0)
}

fn main() {
    let scene = build_scene(
        vec!(light1()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: default_material(Color::WHITE),
                    floor: default_material(Color::WHITE), 
                    front: brick_shader(), 
                    back: default_material(Color::WHITE),
                    left: default_material(Color::WHITE),
                    right: brick_shader(),
                }),
            ),
        ),
    );

    let image = render(scene, image(500, 500), camera([-300.0, 0.0, 300.0], [350.0, -350.0, -350.0]));
    write_to_png( image, "output/main");
}
