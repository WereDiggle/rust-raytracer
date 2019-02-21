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

#[test]
fn many_weirds() {
    let mut scene = Scene::new();
    let weird1 = create_comp_weird(80.0, translation(-150.0, 80.0, 40.0), Color::RED);
    let weird2 = create_comp_weird(60.0, translation(80.0, 60.0, 0.0), Color::BLUE);
    let weird3 = create_comp_weird(40.0, translation(50.0, 40.0, 150.0), Color::GREEN);
    let weird4 = create_comp_weird(20.0, translation(-50.0, 20.0, 150.0), Color::PURPLE);
    let floor = create_floor(600.0, Color::GRAY);
    scene.root.add_child(Box::new(weird1));
    scene.root.add_child(Box::new(weird2));
    scene.root.add_child(Box::new(weird3));
    scene.root.add_child(Box::new(weird4));
    scene.root.add_child(Box::new(floor));

    scene.add_light(Box::new(PointLight::new(dvec3!(-100.0, 300.0, 300.0), Color::new(1.0, 1.0, 1.0), 150000.0, (0.0, 0.0, 1.0*PI))));
    scene.ambient_light = AmbientLight::new(Color::WHITE, 0.0);
    //scene.set_background_from_path("assets/images/backgrounds/forest2.jpg");

    let image = render(scene, image(1280, 720), camera([0.0, 400.0, 400.0], [0.0, 0.0, 0.0]));
    write_to_png( image, "output/many_weirds");
}