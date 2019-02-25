extern crate raytracer;
extern crate euler;

mod common;

use common::*;
use raytracer::*;
use raytracer::matrix::*;
use euler::*;
use std::f64::consts::PI;

#[test]
fn subtract_sphere() {
    let mut scene = Scene::new();
    let mut root = SceneNode::new();
    let sphere1 = Box::new(Sphere::new(50.0));
    let sphere2 = Box::new(Sphere::new(40.0));

    let sphere1 = Box::new(BaseShape::new(DMat4::identity(), sphere1));
    let sphere2 = Box::new(BaseShape::new(translation(0.0, 50.0, 20.0), sphere2));
    let weird = Box::new(SubtractShape::new(sphere1, sphere2));

    root.set_primitive(weird);
    root.set_material(slightly_shiney(Color::LIME));
    scene.root = Box::new(root);

    scene.add_light(Box::new(PointLight::new(dvec3!(100.0, 300.0, 200.0), Color::WHITE, 100000.0, (0.0, 0.0, 4.0*PI))));
    scene.add_light(Box::new(PointLight::new(dvec3!(-100.0, 300.0, 200.0), Color::WHITE, 100000.0, (0.0, 0.0, 4.0*PI))));

    let image = render(scene, image(1280, 720), camera([0.0, 150.0, 100.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/subtract_shape");
}

fn light1() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(-100.0, 300.0, 300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn light2() -> Box<PointLight> {
    Box::new(PointLight::new(dvec3!(100.0, 300.0, -300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI)))
}

fn no_ambient() -> AmbientLight {
    AmbientLight::new(Color::WHITE, 0.0)
}

#[test]
fn many_weirds() {
    let scene = build_scene(
        vec!(light1()),
        no_ambient(),
        "assets/images/backgrounds/forest2.jpg",
        scene_node(
            DMat4::identity(),
            vec!(
                create_comp_weird(80.0, translation(-150.0, 80.0, 40.0), Color::RED),
                create_comp_weird(60.0, translation(80.0, 60.0, 0.0), Color::BLUE),
                create_comp_weird(40.0, translation(50.0, 40.0, 150.0), Color::GREEN),
                create_comp_weird(20.0, translation(-50.0, 20.0, 150.0), Color::PURPLE),
                create_floor(600.0, Color::GRAY),
            ),
        ),
    );

    let image = render(scene, image(1920, 1080), camera([0.0, 300.0, 300.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/many_weirds");
}

fn default_material(color: Color) -> Box<PhongShader> {
    Box::new(PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0))
}

fn clear_material() -> Box<TranslucentShader> {
    Box::new(TranslucentShader::new(Color::WHITE, 1.52))
}

#[test]
fn subtraction_room() {
    let sphere_size: f64 = 80.0;
    let make_spikey_thing = || -> Box<SubtractShape> {
        subtract_shape(
            DMat4::identity(),
            subtract_shape(
                DMat4::identity(),
                subtract_shape(
                    DMat4::identity(),
                    base_shape(
                        DMat4::identity(),
                        sphere(sphere_size),
                    ),
                    base_shape(
                        translation(sphere_size, 0.0, 0.0),
                        sphere(sphere_size),
                    ),
                ),
                base_shape(
                    translation(0.0, sphere_size, 0.0),
                    sphere(sphere_size),
                ),
            ),
            base_shape(
                translation(0.0, 0.0, sphere_size),
                sphere(sphere_size),
            ),
        )
    };

    let make_weird_cube = || -> Box<SubtractShape> {
        subtract_shape(
            DMat4::identity(),
            base_shape(
                DMat4::identity(),
                cube(sphere_size*2.0),
            ),
            base_shape(
                DMat4::identity(),
                sphere(sphere_size),
            ),
        )
    };

    let scene = build_scene(
        vec!(light1(), light2()),
        no_ambient(),
        "assets/images/backgrounds/forest2.jpg",
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
                    translation(-150.0, 0.0, 40.0)*rotation(Axis::Z, 60.0),
                    clear_material(),
                    make_spikey_thing(),
                    vec!(),
                ),
                geometry_node(
                    translation(100.0, 0.0, 40.0)*rotation(Axis::Y, 45.0),
                    clear_material(),
                    make_weird_cube(),
                    vec!(),
                ),
            ),
        ),
    );

    let mut render_config = RenderConfig::default();
    render_config.anti_alias = true;
    let image = render_with_config(scene, image(5000, 5000), camera([0.0, 0.0, 350.0], [0.0, -100.0, -350.0]), render_config);
    write_to_png( image, "output/subtraction_room");
}