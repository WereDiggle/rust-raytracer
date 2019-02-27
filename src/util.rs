use super::*;
use euler::*;

pub fn build_scene(lights: Vec<Box<Lightable + Send + Sync>>, 
                   ambient_light: AmbientLight,
                   background_path: &str,
                   root: Box<Traceable + Send + Sync>) -> Scene {
    
    let mut scene = Scene::new();
    scene.ambient_light = ambient_light;
    for light in lights.into_iter() {
        scene.add_light(light);
    }
    scene.set_background_from_path(background_path);
    scene.root = root;
    scene
}

pub fn default_node() -> Box<SceneNode> {
    Box::new(SceneNode::new())
}

pub fn geometry_node(transform: DMat4, 
                  material: Box<Shadable + Send + Sync>,
                  primitive: Box<Intersectable + Send + Sync>,
                  children: Vec<Box<Traceable + Send + Sync>>) -> Box<SceneNode> {
    let mut node = SceneNode::new();
    node.set_transform(transform);
    node.set_material(material);
    node.set_primitive(primitive);
    for child in children.into_iter() {
        node.add_child(child);
    }
    Box::new(node)
}

pub fn scene_node(transform: DMat4, 
                  children: Vec<Box<Traceable + Send + Sync>>) -> Box<SceneNode> {
    let mut node = SceneNode::new();
    node.set_transform(transform);
    for child in children.into_iter() {
        node.add_child(child);
    }
    Box::new(node)
}

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

pub fn create_floor(size: f64, color: Color) -> Box<SceneNode> {
    let mut floor = SceneNode::new();
    floor.set_primitive(Box::new(RectangularPlane::new(size, size)));
    let mut comp = CompositeShader::new();
    comp.add_shader(0.8, Box::new(PhongShader::new(color*1.0, color*0.0, color*0.1, 1.0)));
    comp.add_shader(0.2, Box::new(ReflectionShader::new(color*1.0)));
    floor.set_material(Box::new(comp));
    floor.set_transform(rotation(Axis::X, -90.0));
    Box::new(floor)
}

pub fn create_mirror(size: f64, color: Color, transform: DMat4) -> SceneNode {
    let mut wall = SceneNode::new();
    wall.set_primitive(Box::new(RectangularPlane::new(size, size)));
    wall.set_material(Box::new(ReflectionShader::new(color*1.0)));
    wall.set_transform(transform);
    wall
}

#[derive(Clone, Copy)]
pub struct RoomColorScheme {
    pub ceiling: Color,
    pub floor: Color,
    pub front: Color,
    pub back: Color,
    pub left: Color,
    pub right: Color,
}

pub fn create_room(size: f64, room_color: RoomColorScheme) -> Box<SceneNode> {
    let mut interior_box = SceneNode::new();

    let ceiling = create_wall(size*1.01, room_color.ceiling, rotation(Axis::X, -90.0) * translation(0.0, 0.0, -size/2.0));
    let floor = create_wall(size*1.01, room_color.floor, rotation(Axis::X, 90.0) * translation(0.0, 0.0, -size/2.0));

    let front = create_wall(size*1.01, room_color.front, translation(0.0, 0.0, -size/2.0));
    let back = create_wall(size*1.01, room_color.back, rotation(Axis::X, 180.0) * translation(0.0, 0.0, -size/2.0));

    let left = create_wall(size*1.01, room_color.left, rotation(Axis::Y, 90.0) * translation(0.0, 0.0, -size/2.0));
    let right = create_wall(size*1.01, room_color.right, rotation(Axis::Y, -90.0) * translation(0.0, 0.0, -size/2.0));

    interior_box.add_child(Box::new(ceiling));
    interior_box.add_child(Box::new(floor));
    interior_box.add_child(Box::new(front));
    interior_box.add_child(Box::new(back));
    interior_box.add_child(Box::new(left));
    interior_box.add_child(Box::new(right));

    Box::new(interior_box)
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

pub fn subtract_shape(transform: DMat4, positive: Box<Compositable + Send + Sync>, negative: Box<Compositable + Send + Sync>) -> Box<SubtractShape> {
    let mut shape = SubtractShape::new(positive, negative);
    shape.set_transform(transform);
    Box::new(shape)
}

pub fn or_shape(transform: DMat4, primary: Box<Compositable + Send + Sync>, secondary: Box<Compositable + Send + Sync>) -> Box<OrShape> {
    let mut shape = OrShape::new(primary, secondary);
    shape.set_transform(transform);
    Box::new(shape)
}

pub fn base_shape(transform: DMat4, primitive: Box<Intersectable + Send + Sync>) -> Box<BaseShape> {
    Box::new(BaseShape::new(transform, primitive))
}

pub fn sphere(radius: f64) -> Box<Sphere> {
    Box::new(Sphere::new(radius))
}

pub fn cube(length: f64) -> Box<Cube> {
    Box::new(Cube::new(length))
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

pub fn create_comp_weird(size: f64, transform: DMat4, color: Color) -> Box<SceneNode> {
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
    Box::new(weird)
}

pub fn create_sphere(size: f64, transform: DMat4, color: Color) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(Sphere::new(size)));
    sphere.set_material(Box::new(PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0)));
    sphere.set_transform(transform);
    sphere
}
