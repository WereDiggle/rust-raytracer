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
fn main() {
    let sphere_size: f64 = 80.0;
    let scene = build_scene(
        vec!(light1(), light2()),
        no_ambient(),
        Some(SkyBox::from_path(
            "assets/images/backgrounds/building.jpg",
            rotation(Axis::Y, 60.0),
        )),
        scene_node(
            DMat4::identity(),
            vec!(
                create_room(700.0, RoomColorScheme {
                    ceiling: Color::RED,
                    floor: Color::GREEN,
                    front: Color::BLUE,
                    back: Color::CYAN,
                    left: Color::MAGENTA,
                    right: Color::YELLOW,
                }),
                geometry_node(
                    translation(-150.0, 0.0, 40.0)*rotation(Axis::Y, -45.0),
                    diffuse_material(),
                    make_half_sphere(sphere_size),
                    vec!(),
                ),
                geometry_node(
                    translation(100.0, 0.0, 40.0)*rotation(Axis::Y, 45.0),
                    diffuse_material(),
                    make_weird_cube(sphere_size),
                    vec!(),
                ),
            ),
        ),
    );

    let mut render_config = RenderConfig::default();
    render_config.anti_alias = true;
    let image = render_with_config(scene, image(256, 256), camera([0.0, 0.0, 350.0], [0.0, -100.0, -350.0]), render_config);
    write_to_png( image, "output/main");
}
