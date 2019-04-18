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

fn default_material(color: Color) -> Box<PhongShader> {
    PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0)
}

#[test]
fn texture_room() {
    let scene = build_scene(
        vec!(light1(), light2(), light3()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: texture_phong_material("assets/images/textures/granite.jpg", 1.0, 0.0, 0.0, 2.0),
                    floor: texture_phong_material("assets/images/textures/wood_boards.jpg", 1.0, 0.0, 0.0, 1.0),
                    front: texture_phong_material("assets/images/textures/brick_wall.jpg", 1.0, 0.0, 0.0, 1.0),
                    back: default_material(Color::CYAN),
                    left: texture_phong_material("assets/images/textures/orange_leather.jpg", 1.0, 0.0, 0.0, 1.0),
                    right: ReflectionShader::new(Color::WHITE),
                }),
                geometry_node(
                    translation(-150.0, -270.0, 0.0),
                    texture_phong_material("assets/images/textures/orange_leather.jpg", 1.0, 0.0, 0.0, 1.0),
                    Sphere::from_radius(80.0),
                    vec!(),
                ),
                geometry_node(
                    translation(150.0, -270.0, 80.0),
                    texture_phong_material("assets/images/textures/cube_rgb_gradient.png", 0.5, 0.5, 0.0, 4.0),
                    Cube::new(160.0),
                    vec!(),
                ),
            ),
        ),
    );

    let image = render(scene, image(5000, 5000), camera([-300.0, 0.0, 300.0], [350.0, -350.0, -350.0]));
    write_to_png( image, "output/texture_room");
}

#[test]
fn texture_cube() {
    let scene = build_scene(
        vec!(light1(), light2(), light3(), light4()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: default_material(Color::WHITE),
                    floor: ReflectionShader::new(Color::WHITE),
                    front: ReflectionShader::new(Color::WHITE),
                    back: default_material(Color::CYAN),
                    left: default_material(Color::YELLOW),
                    right: ReflectionShader::new(Color::WHITE),
                }),
                geometry_node(
                    translation(0.0, 0.0, 0.0),
                    texture_phong_material("assets/images/textures/cube_rgb_numbers.png", 0.5, 0.5, 0.0, 4.0),
                    Cube::new(160.0),
                    vec!(),
                ),
            ),
        ),
    );

    let image = render(scene, image(1920, 1080), camera([-310.0, 300.0, 300.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/texture_cube");
}

#[test]
fn skybox_test() {
    let scene = build_scene(
        vec!(),
        no_ambient(),
        Some(SkyBox::from_path("assets/images/living_room.jpg", DMat4::identity())),
        scene_node(
            DMat4::identity(),
            vec!(),
        ),
    );

    let image = render(scene, image(512, 512), camera([-310.0, 300.0, 300.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/skybox_test");
}

#[test]
fn correlated_texture() {
    let scene = build_scene(
        vec!(light1(), light2(), light3()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                create_room_from_material(700.0, RoomMaterialScheme {
                    ceiling: texture_phong_material("assets/images/tiny.png", 4.0, 6.0, 0.0, 2.0),
                    floor: texture_phong_material("assets/images/tiny.png", 1.0, 0.0, 0.0, 1.0),
                    front: texture_phong_material("assets/images/tiny.png", 1.0, 0.0, 0.0, 1.0),
                    back: default_material(Color::CYAN),
                    left: texture_phong_material("assets/images/tiny.png", 1.0, 0.0, 0.0, 1.0),
                    right: texture_phong_material("assets/images/tiny.png", 1.0, 0.0, 0.0, 1.0),
                }),
            ),
        ),
    );

    let image = render(scene, image(512, 512), camera([-300.0, 0.0, 300.0], [350.0, -350.0, -350.0]));
    write_to_png( image, "output/correlated_texture");
}

#[test]
fn plane_texture() {
    let scene = build_scene(
        vec!(front_light()),
        no_ambient(),
        None,
        scene_node(
            DMat4::identity(),
            vec!(
                geometry_node(
                    translation(0.0, 0.0, 0.0),
                    texture_phong_material("assets/images/textures/keanu.jpg", 1.0, 0.0, 0.0, 4.0),
                    Plane::with_tangent(dvec3!(0.0, 0.0, 0.0), dvec3!(0.0, 0.0, 1.0), dvec3!(0.0, 1.0, 0.0)),
                    vec!(),
                ),
            ),
        ),
    );

    let image = render(scene, image(512, 512), camera([0.0, 0.0, 1.0], [1.0, 0.0, 1.0]));
    write_to_png( image, "output/plane_texture");
}
