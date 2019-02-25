extern crate raytracer;
extern crate euler;

mod common;

use common::*;
use raytracer::*;
use raytracer::matrix::*;
use euler::*;
use std::f64::consts::PI;

#[test]
fn scene_test_1() {
    let mut test_scene = Scene::new();
    let room_size = 200.0;
    test_scene.root = Box::new(create_interior_box(room_size));
    test_scene.add_light(Box::new(PointLight::new(dvec3!(0.0, (room_size/2.0)*0.6, (room_size/2.0)*0.6), Color::new(1.0, 1.0, 1.0), 100000.0, (0.0, 0.0, 4.0*PI))));
    test_scene.ambient_light = AmbientLight::new(Color::WHITE, 1.0);

    let sphere = create_sphere(25.0, translation(0.0, -75.0, 0.0), Color::NAVY);
    test_scene.root.add_child(Box::new(sphere));

    let image = render(test_scene, image(1280, 720), camera([0.0, 0.0, room_size/2.0], [0.0, -(room_size/2.0)*0.6, 0.0]));
    write_to_png( image, "output/scene_1");
}

#[test]
fn scene_test_2() {
    let mut test_scene = Scene::new();
    let room_size = 200.0;
    test_scene.root = Box::new(create_interior_mirror_box(room_size));
    test_scene.add_light(Box::new(PointLight::new(dvec3!(0.0, (room_size/2.0)*0.6, (room_size/2.0)*0.6), Color::new(1.0, 1.0, 1.0), 100000.0, (0.0, 0.0, 4.0*PI))));
    test_scene.ambient_light = AmbientLight::new(Color::WHITE, 1.0);

    let sphere = create_sphere(25.0, translation(0.0, -75.0, 0.0), Color::NAVY);
    test_scene.root.add_child(Box::new(sphere));

    let image = render(test_scene, image(1280, 720), camera([0.0, 0.0, room_size/2.0], [0.0, -(room_size/2.0)*0.6, 0.0]));
    write_to_png( image, "output/scene_2");
}

#[test]
fn scene_test_3() {
    let mut test_scene = Scene::new();
    let room_size = 200.0;
    test_scene.root = Box::new(create_interior_box(room_size));
    test_scene.add_light(Box::new(PointLight::new(dvec3!(0.0, (room_size/2.0)*0.6, (room_size/2.0)*0.6), Color::new(1.0, 1.0, 1.0), 100000.0, (0.0, 0.0, 4.0*PI))));
    test_scene.ambient_light = AmbientLight::new(Color::WHITE, 1.0);

    let sphere = create_sphere(50.0, translation(0.0, -50.0, 0.0), Color::NAVY);
    test_scene.root.add_child(Box::new(sphere));

    let image = render(test_scene, image(1280, 720), camera([0.0, -50.0, 0.0], [0.0, 0.0, -(room_size/2.0)]));
    write_to_png( image, "output/scene_3");
}

#[test]
fn scene_test_4() {
    let mut test_scene = Scene::new();
    let room_size = 200.0;
    test_scene.root = Box::new(create_interior_box(room_size));
    test_scene.add_light(Box::new(PointLight::new(dvec3!(0.0, (room_size/2.0)*0.6, (room_size/2.0)*0.6), Color::new(1.0, 1.0, 1.0), 100000.0, (0.0, 0.0, 4.0*PI))));
    test_scene.ambient_light = AmbientLight::new(Color::WHITE, 1.0);

    let sphere = create_translucent_sphere(50.0, translation(0.0, -50.0, 0.0), Color::WHITE, 1.52);
    test_scene.root.add_child(Box::new(sphere));

    let image = render(test_scene, image(1280, 720), camera([0.0, 0.0, room_size/2.0], [0.0, -(room_size/2.0)*0.6, 0.0]));
    write_to_png( image, "output/scene_4");
}

#[test]
fn scene_test_5() {
    let mut test_scene = Scene::new();
    let room_size = 200.0;
    test_scene.root = Box::new(create_interior_box(room_size));
    test_scene.add_light(Box::new(PointLight::new(dvec3!(0.0, (room_size/2.0)*0.6, (room_size/2.0)*0.6), Color::new(1.0, 1.0, 1.0), 100000.0, (0.0, 0.0, 4.0*PI))));
    test_scene.ambient_light = AmbientLight::new(Color::WHITE, 1.0);

    let sphere = create_cube(50.0, translation(0.0, -75.0, 0.0), Color::WHITE);
    test_scene.root.add_child(Box::new(sphere));

    let image = render(test_scene, image(512, 512), camera([0.0, 0.0, room_size/2.0], [0.0, -(room_size/2.0)*0.6, 0.0]));
    write_to_png( image, "output/scene_5");
}

#[test]
fn refraction_test() {
    for i in 0..11 {
        let mut test_scene = Scene::new();
        let room_size = 200.0;
        test_scene.root = Box::new(create_interior_box(room_size));
        test_scene.add_light(Box::new(PointLight::new(dvec3!(0.0, (room_size/2.0)*0.6, (room_size/2.0)*0.6), Color::new(1.0, 1.0, 1.0), 100000.0, (0.0, 0.0, 4.0*PI))));
        test_scene.ambient_light = AmbientLight::new(Color::WHITE, 1.0);

        let sphere = create_translucent_sphere(25.0, translation(0.0, -50.0, 50.0), Color::WHITE, 1.0 + i as f64 * 0.3);
        test_scene.root.add_child(Box::new(sphere));

        let image = render(test_scene, image(512, 512), camera([0.0, -50.0, room_size/2.0], [0.0, -50.0, 0.0]));
        write_to_png( image, &format!("output/refraction/refraction_{:02}", i));
    }
}

#[test]
fn scene_light_1() {
    for i in 1..11 {
        let mut test_scene = Scene::new();
        let room_size = 200.0;
        test_scene.root = Box::new(create_interior_box(room_size));
        test_scene.add_light(Box::new(PointLight::new(dvec3!((room_size/3.0), (room_size/2.0)*0.6, (room_size/3.0)), Color::new(1.0, 1.0, 1.0), 2000.0 * i as f64, (0.0, 0.0, 4.0*PI))));
        test_scene.add_light(Box::new(PointLight::new(dvec3!(-(room_size/3.0), (room_size/2.0)*0.6, (room_size/3.0)), Color::new(1.0, 1.0, 1.0), 2000.0 * i as f64, (0.0, 0.0, 4.0*PI))));
        test_scene.add_light(Box::new(PointLight::new(dvec3!((room_size/3.0), (room_size/2.0)*0.6, -(room_size/3.0)), Color::new(1.0, 1.0, 1.0), 2000.0 * i as f64, (0.0, 0.0, 4.0*PI))));
        test_scene.add_light(Box::new(PointLight::new(dvec3!(-(room_size/3.0), (room_size/2.0)*0.6, -(room_size/3.0)), Color::new(1.0, 1.0, 1.0), 2000.0 * i as f64, (0.0, 0.0, 4.0*PI))));

        let image = render(test_scene, image(512, 512), camera([0.0, 0.0, 200.0], [0.0; 3]));
        write_to_png( image, &format!("output/lighting/light_scene_{:02}", i));
    }
}

#[test]
fn background_1() {
    let mut scene = Scene::new();
    let sphere = create_translucent_sphere(50.0, translation(-60.0, 0.0, 0.0), Color::WHITE, 1.52);
    scene.root.add_child(Box::new(sphere));
    let sphere2 = create_mirror_sphere(50.0, translation(60.0, 0.0, 0.0), Color::WHITE);
    scene.root.add_child(Box::new(sphere2));
    scene.add_light(Box::new(PointLight::new(dvec3!(0.0, 60, 60), Color::new(1.0, 1.0, 1.0), 100000.0, (0.0, 0.0, 4.0*PI))));
    scene.ambient_light = AmbientLight::new(Color::WHITE, 1.0);
    scene.set_background_from_path("assets/images/backgrounds/room.jpg");

    println!("after setting background");
    let image = render(scene, image(512, 512), camera([0.0, 0.0, 200.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/background_1");
}

#[test]
fn many_balls() {
    let mut scene = Scene::new();
    let sphere1 = create_comp_sphere(80.0, translation(-150.0, 80.0, 40.0), Color::RED);
    let sphere2 = create_comp_sphere(60.0, translation(80.0, 60.0, 0.0), Color::BLUE);
    let sphere3 = create_comp_sphere(40.0, translation(50.0, 40.0, 150.0), Color::GREEN);
    let sphere4 = create_comp_sphere(20.0, translation(-50.0, 20.0, 150.0), Color::PURPLE);
    let floor = create_floor(600.0, Color::GRAY);
    scene.root.add_child(Box::new(sphere1));
    scene.root.add_child(Box::new(sphere2));
    scene.root.add_child(Box::new(sphere3));
    scene.root.add_child(Box::new(sphere4));
    scene.root.add_child(floor);

    scene.add_light(Box::new(PointLight::new(dvec3!(-100.0, 300.0, 300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI))));
    scene.ambient_light = AmbientLight::new(Color::WHITE, 0.0);
    scene.set_background_from_path("assets/images/backgrounds/forest2.jpg");

    let image = render(scene, image(1920, 1080), camera([0.0, 200.0, 400.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/many_balls");
}

#[test]
fn translucent_shadow() {
    for i in 9..10 {
        let mut scene = Scene::new();
        let sphere1 = create_translucent_cube(160.0, translation(0.0, 100.0, 0.0) * scaling(1.0, 1.0, 1.0/160.0 * i as f64), Color::WHITE - Color::LIGHT_BLACK, 1.52);
        let floor = create_floor(600.0, Color::WHITE);
        scene.root.add_child(Box::new(sphere1));
        scene.root.add_child(floor);

        scene.add_light(Box::new(PointLight::new(dvec3!(-100.0, 300.0, -300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI))));
        scene.set_background_from_path("assets/images/backgrounds/forest2.jpg");

        let image = render(scene, image(1920, 1080), camera([0.0, 200.0, 400.0], [0.0, 0.0, 0.0]));
        write_to_png( image, &format!("output/translucent_shadow_{:02}", i));
    }
}