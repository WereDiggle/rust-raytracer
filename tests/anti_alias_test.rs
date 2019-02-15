extern crate raytracer;
extern crate euler;

mod common;

use common::*;
use raytracer::*;
use raytracer::matrix::*;
use euler::*;
use std::f64::consts::PI;

#[test]
fn anti_alias_comparison() {
    let mut test_scene = Scene::new();
    let room_size = 200.0;
    test_scene.root = Box::new(create_interior_box(room_size));
    test_scene.add_light(Box::new(PointLight::new(dvec3!(0.0, (room_size/2.0)*0.6, (room_size/2.0)*0.6), Color::new(1.0, 1.0, 1.0), 100000.0, (0.0, 0.0, 4.0*PI))));
    test_scene.ambient_light = AmbientLight::new(Color::WHITE, 1.0);

    let sphere = create_cube(50.0, translation(0.0, -75.0, 0.0), Color::WHITE);
    test_scene.root.add_child(Box::new(sphere));


    let image1 = render(test_scene.clone(), image(512, 512), camera([0.0, 0.0, room_size/2.0], [0.0, -(room_size/2.0)*0.6, 0.0]));
    write_to_png( image1, "output/anti_alias_0");

    let mut render_config = RenderConfig::default();
    render_config.anti_alias = true;
    let image2 = render_with_config(test_scene, image(512, 512), camera([0.0, 0.0, room_size/2.0], [0.0, -(room_size/2.0)*0.6, 0.0]), render_config);
    write_to_png( image2, "output/anti_alias_1");
}
