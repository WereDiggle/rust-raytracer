extern crate raytracer;
extern crate euler;

use raytracer::*;
use raytracer::matrix::*;
use euler::*;

fn create_wall(size: f64, color: Color) -> SceneNode {
    let wall = SceneNode::new();
    wall.set_primitive(Box::new(RectangularPlane::new(size, size)));
    wall.set_material(Box::new(PhongShader::new(Color::white)));
    wall
}

fn create_interior_box(size: f64) -> SceneNode {
    let interior_box = SceneNode::new();

    let floor = 
}

fn shadow_test_1() {
    let mut test_scene = setup_scene();
    let test_material = PhongShader::new(Color::white()*0.1, Color::white()*0.9, Color::white()*0.1, 4.0);
    test_scene.add_light(PointLight::new(dvec3!(200.0, 200.0, 200.0), Color::new(1.0, 1.0, 1.0), 1.0, (1.0, 0.0, 0.0)));
    test_scene.root.set_material(Box::new(test_material));
    test_scene.root.set_transform(translation(-1.0, -1.0, -1.0));

    let mut small_sphere = SceneNode::new();
    small_sphere.set_primitive(Box::new(Sphere::new(0.5)));
    small_sphere.set_transform(translation(1.0, 1.0, 1.0));
    test_scene.root.add_child(Box::new(small_sphere));

    let image = render(test_scene, square_image(512), setup_camera());
    write_to_png( image, "output/shadow_1");
}