extern crate raytracer;
extern crate euler;

use raytracer::*;
use raytracer::matrix::*;
use euler::*;

pub fn image(width: u32, height: u32) -> ImageDimension {
    ImageDimension{width, height}
}

pub fn square_image(side: u32) -> ImageDimension {
    ImageDimension{width: side, height: side}
}

pub fn camera(origin: [f64; 3], target: [f64; 3]) -> CameraConfig {
    CameraConfig { origin: dvec3!(origin), target: dvec3!(target), up: dvec3!(0.0, 1.0, 0.0), fov_y: 90.0}
}

pub fn basic_diffuse(color: Color) -> Box<PhongShader> {
    return Box::new(PhongShader::new(color*1.0, color*0.0, color*0.0, 1.0));
}

pub fn slightly_shiney(color: Color) -> Box<PhongShader> {
    return Box::new(PhongShader::new(color*1.0, color*0.0, color*0.0, 1.0));
}

pub fn create_wall(size: f64, color: Color, transform: DMat4) -> SceneNode {
    let mut wall = SceneNode::new();
    wall.set_primitive(Box::new(RectangularPlane::new(size, size)));
    wall.set_material(Box::new(PhongShader::new(color*1.0, color*0.0, color*0.0, 1.0)));
    wall.set_transform(transform);
    wall
}

pub fn create_floor(size: f64, color: Color) -> SceneNode {
    let mut floor = SceneNode::new();
    floor.set_primitive(Box::new(RectangularPlane::new(size, size)));
    let mut comp = CompositeShader::new();
    comp.add_shader(0.8, Box::new(PhongShader::new(color*1.0, color*0.0, color*0.1, 1.0)));
    comp.add_shader(0.2, Box::new(ReflectionShader::new(color*1.0)));
    floor.set_material(Box::new(comp));
    floor.set_transform(rotation(Axis::X, -90.0));
    floor
}

pub fn create_mirror(size: f64, color: Color, transform: DMat4) -> SceneNode {
    let mut wall = SceneNode::new();
    wall.set_primitive(Box::new(RectangularPlane::new(size, size)));
    wall.set_material(Box::new(ReflectionShader::new(color*1.0)));
    wall.set_transform(transform);
    wall
}

pub fn create_interior_box(size: f64) -> SceneNode {
    let mut interior_box = SceneNode::new();

    let ceiling = create_wall(size*1.01, Color::GREEN, rotation(Axis::X, -90.0) * translation(0.0, 0.0, -size/2.0));
    let floor = create_wall(size*1.01, Color::CHOCOLATE, rotation(Axis::X, 90.0) * translation(0.0, 0.0, -size/2.0));

    let front = create_wall(size*1.01, Color::ROSY_BROWN, translation(0.0, 0.0, -size/2.0));
    let back = create_wall(size*1.01, Color::MISTY_ROSE, rotation(Axis::X, 180.0) * translation(0.0, 0.0, -size/2.0));

    let left = create_wall(size*1.01, Color::RED, rotation(Axis::Y, 90.0) * translation(0.0, 0.0, -size/2.0));
    let right = create_wall(size*1.01, Color::BLUE, rotation(Axis::Y, -90.0) * translation(0.0, 0.0, -size/2.0));

    interior_box.add_child(Box::new(ceiling));
    interior_box.add_child(Box::new(floor));
    interior_box.add_child(Box::new(front));
    interior_box.add_child(Box::new(back));
    interior_box.add_child(Box::new(left));
    interior_box.add_child(Box::new(right));

    interior_box
}

pub fn create_interior_mirror_box(size: f64) -> SceneNode {
    let mut interior_box = SceneNode::new();

    let ceiling = create_wall(size*1.01, Color::GREEN, rotation(Axis::X, -90.0) * translation(0.0, 0.0, -size/2.0));
    let floor = create_wall(size*1.01, Color::CHOCOLATE, rotation(Axis::X, 90.0) * translation(0.0, 0.0, -size/2.0));

    let front = create_mirror(size*1.01, Color::WHITE, translation(0.0, 0.0, -size/2.0));
    let back = create_wall(size*1.01, Color::MISTY_ROSE, rotation(Axis::X, 180.0) * translation(0.0, 0.0, -size/2.0));

    let left = create_mirror(size*1.01, Color::WHITE, rotation(Axis::Y, 90.0) * translation(0.0, 0.0, -size/2.0));
    let right = create_mirror(size*1.01, Color::WHITE, rotation(Axis::Y, -90.0) * translation(0.0, 0.0, -size/2.0));

    interior_box.add_child(Box::new(ceiling));
    interior_box.add_child(Box::new(floor));
    interior_box.add_child(Box::new(front));
    interior_box.add_child(Box::new(back));
    interior_box.add_child(Box::new(left));
    interior_box.add_child(Box::new(right));

    interior_box
}

pub fn create_cube(size: f64, transform: DMat4, color: Color) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(Cube::new(size)));
    sphere.set_material(Box::new(PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0)));
    sphere.set_transform(transform);
    sphere
}

pub fn create_translucent_cube(size: f64, transform: DMat4, color: Color, refractive_index: f64) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(Cube::new(size)));
    sphere.set_material(Box::new(TranslucentShader::new(color*1.0, refractive_index)));
    sphere.set_transform(transform);
    sphere
}

pub fn create_translucent_sphere(size: f64, transform: DMat4, color: Color, refractive_index: f64) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(OneWay::new(Box::new(Sphere::new(size)))));
    sphere.set_material(Box::new(TranslucentShader::new(color*1.0, refractive_index)));
    sphere.set_transform(transform);
    sphere
}

pub fn create_mirror_sphere(size: f64, transform: DMat4, color: Color) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(OneWay::new(Box::new(Sphere::new(size)))));
    sphere.set_material(Box::new(ReflectionShader::new(color*1.0)));
    sphere.set_transform(transform);
    sphere
}

pub fn create_comp_sphere(size: f64, transform: DMat4, color: Color) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(OneWay::new(Box::new(Sphere::new(size)))));
    let phong = Box::new(PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0));
    //let glass = Box::new(TranslucentShader::new(Color::WHITE, 1.52));
    let reflect = Box::new(ReflectionShader::new(Color::WHITE));
    let mut comp = Box::new(CompositeShader::new());
    comp.add_shader(0.8, phong);
    comp.add_shader(0.2, reflect);
    sphere.set_material(comp);
    sphere.set_transform(transform);
    sphere
}

pub fn create_weird(size: f64, offset: f64) -> Box<SubtractShape> {
    let sphere1 = Box::new(Sphere::new(size));
    let sphere2 = Box::new(Sphere::new(size));
    let sphere3 = Box::new(Sphere::new(size));
    let sphere4 = Box::new(Sphere::new(size));

    let sphere1 = Box::new(BaseShape::new(DMat4::identity(), sphere1));
    let sphere2 = Box::new(BaseShape::new(translation(0.0, offset, 0.0), sphere2));
    let sphere3 = Box::new(BaseShape::new(translation(0.0, 0.0, offset), sphere3));
    let sphere4 = Box::new(BaseShape::new(translation(offset, 0.0, 0.0), sphere4));
    let sphere1 = Box::new(SubtractShape::new(sphere1, sphere2));
    let sphere1 = Box::new(SubtractShape::new(sphere1, sphere3));
    Box::new(SubtractShape::new(sphere1, sphere4))
}

pub fn create_comp_weird(size: f64, transform: DMat4, color: Color) -> SceneNode {
    let mut weird = SceneNode::new();
    weird.set_primitive(create_weird(size, size));
    let phong = Box::new(PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0));
    //let glass = Box::new(TranslucentShader::new(Color::WHITE, 1.52));
    let reflect = Box::new(ReflectionShader::new(Color::WHITE));
    let mut comp = Box::new(CompositeShader::new());
    comp.add_shader(0.8, phong);
    comp.add_shader(0.2, reflect);
    weird.set_material(comp);
    weird.set_transform(transform);
    weird
}

pub fn create_sphere(size: f64, transform: DMat4, color: Color) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(Sphere::new(size)));
    sphere.set_material(Box::new(PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0)));
    sphere.set_transform(transform);
    sphere
}
