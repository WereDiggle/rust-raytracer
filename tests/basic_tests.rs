extern crate raytracer;
extern crate euler;

mod common;

use common::*;
use raytracer::*;
use raytracer::matrix::*;
use euler::*;

fn setup_scene() -> Scene {
    let mut test_scene = Scene::new();
    test_scene.root.set_primitive(Box::new(Sphere::new(100.0)));
    test_scene.add_light(PointLight::new(dvec3!(0.0, 1000.0, 200.0), Color::new(0.0, 1.0, 1.0), 1.0, (1.0, 0.0, 0.0)));
    test_scene
}

fn setup_camera() -> CameraConfig {
    CameraConfig{ origin: dvec3!(0.0, 0.0, 200.0),
                  target: dvec3!(0.0, 0.0, 0.0),
                  up: dvec3!(0.0, 1.0, 0.0),
                  fov_y: 90.0}
}

#[test]
fn white_sphere_one_light() {
    let mut test_scene = setup_scene();
    let test_material = PhongShader::new(Color::WHITE*1.0, Color::WHITE*0.0, Color::WHITE*0.1, 1.0);
    test_scene.root.set_material(Box::new(test_material));
    let image = render(test_scene, square_image(512), setup_camera());
    write_to_png( image, "output/test1");
}

#[test]
fn phong_test_1() {
    let mut test_scene = setup_scene();
    let test_material = PhongShader::new(Color::WHITE*0.1, Color::WHITE*0.9, Color::WHITE*0.1, 2.0);
    test_scene.add_light(PointLight::new(dvec3!(200.0, 200.0, 200.0), Color::new(1.0, 1.0, 1.0), 1.0, (1.0, 0.0, 0.0)));
    test_scene.root.set_material(Box::new(test_material));
    let image = render(test_scene, square_image(512), setup_camera());
    write_to_png( image, "output/phong1");
}

#[test]
fn phong_test_2() {
    let mut test_scene = setup_scene();
    let test_material = PhongShader::new(Color::WHITE*0.1, Color::WHITE*0.9, Color::WHITE*0.1, 4.0);
    test_scene.add_light(PointLight::new(dvec3!(200.0, 200.0, 200.0), Color::new(1.0, 1.0, 1.0), 1.0, (1.0, 0.0, 0.0)));
    test_scene.root.set_material(Box::new(test_material));
    let image = render(test_scene, square_image(512), setup_camera());
    write_to_png( image, "output/phong2");
}

#[test]
fn plane_test_1() {
    let mut test_scene = setup_scene();
    let test_material = PhongShader::new(Color::WHITE*0.1, Color::WHITE*0.9, Color::WHITE*0.1, 4.0);
    test_scene.root.set_material(Box::new(test_material));
    test_scene.root.set_primitive(Box::new(RectangularPlane::new(100.0, 100.0)));
    test_scene.add_light(PointLight::new(dvec3!(0.0, 0.0, 200.0), Color::new(1.0, 1.0, 1.0), 1.0, (1.0, 0.0, 0.0)));
    let camera_config = CameraConfig{ origin: dvec3!(50.0, 50.0, 200.0),
                                      target: dvec3!(0.0, 0.0, 0.0),
                                      up: dvec3!(0.0, 1.0, 0.0),
                                      fov_y: 90.0};
    let image = render(test_scene, square_image(512), camera_config);
    write_to_png( image, "output/plane1");
}

fn setup_diffuse_cube(transform: DMat4) -> Scene {
    let mut scene = Scene::new();
    let material = PhongShader::new(Color::WHITE*0.9, Color::WHITE*0.0, Color::WHITE*0.1, 4.0);
    scene.root.set_transform(transform);
    scene.root.set_material(Box::new(material));
    scene.root.set_primitive(Box::new(Cube::new(100.0)));
    scene.add_light(PointLight::new(dvec3!(200.0, 300.0, 400.0), Color::new(1.0, 1.0, 1.0), 2.0, (1.0, 0.0, 0.0)));
    scene
}

#[test]
fn cube_test_1() {
    let test_scene = setup_diffuse_cube(DMat4::identity());
    let camera_config = camera([50.0, 50.0, 200.0], [0.0; 3]);
    let image = render(test_scene, square_image(512), camera_config);
    write_to_png( image, "output/cube1");
}

#[test]
fn translate_test_1() {
    let test_scene = setup_diffuse_cube(translation(1.0, 0.0, 0.0));
    let camera_config = camera([50.0, 50.0, 200.0], [0.0; 3]);
    let image = render(test_scene, square_image(512), camera_config);
    write_to_png( image, "output/translate_1");
}

#[test]
fn rotate_test_1() {
    let test_scene = setup_diffuse_cube(rotation(Axis::Z, 15.0));
    let camera_config = camera([50.0, 50.0, 200.0], [0.0; 3]);
    let image = render(test_scene, square_image(512), camera_config);
    write_to_png( image, "output/rotate_1");
}

#[test]
fn scale_test_1() {
    let test_scene = setup_diffuse_cube(scaling(2.9, 1.5, 0.3));
    let camera_config = camera([50.0, 50.0, 200.0], [0.0; 3]);
    let image = render(test_scene, square_image(512), camera_config);
    write_to_png( image, "output/scale_1");
}

fn setup_multiple_cubes_same_transform(num_cubes: u32, transform: DMat4) -> Scene {
    let mut scene = Scene::new();
    scene.add_light(PointLight::new(dvec3!(200.0, 300.0, 400.0), Color::new(1.0, 1.0, 1.0), 2.0, (1.0, 0.0, 0.0)));

    let mut base_node = SceneNode::new();
    let material = PhongShader::new(Color::WHITE*0.9, Color::WHITE*0.0, Color::WHITE*0.1, 4.0);
    base_node.set_transform(transform);
    base_node.set_material(Box::new(material));
    base_node.set_primitive(Box::new(Cube::new(50.0)));

    let mut prev_node;
    let mut scene_node = base_node.clone();
    for _i in 1..num_cubes {
        prev_node = scene_node;
        scene_node = base_node.clone();
        scene_node.add_child(Box::new(prev_node.clone()));
    }
    scene.root = scene_node;
    scene
}

#[test]
fn child_transform_test_1() {
    let test_scene = setup_multiple_cubes_same_transform(5, rotation(Axis::Z, 15.0) * translation(50.0, 0.0, 0.0));
    let camera_config = camera([0.0, 0.0, 200.0], [0.0; 3]);
    let image = render(test_scene, square_image(512), camera_config);
    write_to_png( image, "output/child_transform_1");
}

#[test]
fn shadow_test_1() {
    let mut test_scene = setup_scene();
    let test_material = PhongShader::new(Color::WHITE*0.1, Color::WHITE*0.9, Color::WHITE*0.1, 4.0);
    test_scene.add_light(PointLight::new(dvec3!(200.0, 200.0, 200.0), Color::new(1.0, 1.0, 1.0), 1.0, (1.0, 0.0, 0.0)));
    test_scene.root.set_material(Box::new(test_material));
    test_scene.root.set_transform(translation(-100.0, -100.0, -100.0));

    let mut small_sphere = SceneNode::new();
    small_sphere.set_primitive(Box::new(Sphere::new(50.0)));
    small_sphere.set_transform(translation(100.0, 100.0, 100.0));
    test_scene.root.add_child(Box::new(small_sphere));

    let image = render(test_scene, square_image(512), setup_camera());
    write_to_png( image, "output/shadow_1");
}