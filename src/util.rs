use super::*;
use euler::*;

pub fn texture_phong_material(path: &str, diffuse: f64, specular: f64, ambient: f64, shininess: f64) -> Box<MixShader> {
    MixShader::from_shaders(
        vec!(
            TextureShader::new(ImageTexture::from_path(path)),
            PhongShader::new(Color::WHITE*diffuse, Color::WHITE*specular, Color::WHITE*ambient, shininess),
        ),
    )
}

pub fn brick_shader() -> Box<ChainShader> {
    let mut brick_shader = ChainShader::new();
    brick_shader.push_shader(
        NormalMapShader::new(
            NormalMap::from_path("assets/images/normal_maps/brick_wall.jpg")
        )
    );
    /*
    brick_shader.push_shader(
        texture_phong_material("assets/images/textures/brick_wall.jpg", 0.9, 0.1, 0.0, 2.0)
    );
    */
    brick_shader.push_shader(
        PhongShader::new(Color::WHITE*0.9, Color::WHITE*0.1, Color::WHITE*0.0, 2.0)
    );
    brick_shader
}

pub fn no_ambient() -> AmbientLight {
    AmbientLight::new(Color::WHITE, 0.0)
}

pub fn build_scene(lights: Vec<Box<Lightable + Send + Sync>>, 
                   ambient_light: AmbientLight,
                   background: Option<SkyBox>,
                   root: Box<Traceable + Send + Sync>) -> Scene {
    
    let mut scene = Scene::new();
    scene.ambient_light = ambient_light;
    for light in lights.into_iter() {
        scene.add_light(light);
    }
    if let Some(background) = background {
        scene.set_background(background);
    }
    
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
    return PhongShader::new(color*1.0, color*0.0, color*0.0, 1.0);
}

pub fn slightly_shiney(color: Color) -> Box<PhongShader> {
    return PhongShader::new(color*1.0, color*0.0, color*0.0, 1.0);
}

pub fn create_wall_from_material(size: f64, material: Box<Shadable + Send + Sync>, transform: DMat4) -> SceneNode {
    let mut wall = SceneNode::new();
    wall.set_primitive(Box::new(RectangularPlane::new(size, size)));
    wall.set_material(material);
    wall.set_transform(transform);
    wall
}

pub fn create_wall(size: f64, color: Color, transform: DMat4) -> SceneNode {
    let mut wall = SceneNode::new();
    wall.set_primitive(Box::new(RectangularPlane::new(size, size)));
    wall.set_material(PhongShader::new(color*1.0, color*0.0, color*0.0, 1.0));
    wall.set_transform(transform);
    wall
}

pub fn create_floor(size: f64, color: Color) -> Box<SceneNode> {
    let mut floor = SceneNode::new();
    floor.set_primitive(Box::new(RectangularPlane::new(size, size)));
    let mut comp = CompositeShader::new();
    comp.add_shader(0.8, PhongShader::new(color*1.0, color*0.0, color*0.1, 1.0));
    comp.add_shader(0.2, ReflectionShader::new(color*1.0));
    floor.set_material(comp);
    floor.set_transform(rotation(Axis::X, -90.0));
    Box::new(floor)
}

pub fn create_mirror(size: f64, color: Color, transform: DMat4) -> SceneNode {
    let mut wall = SceneNode::new();
    wall.set_primitive(Box::new(RectangularPlane::new(size, size)));
    wall.set_material(ReflectionShader::new(color*1.0));
    wall.set_transform(transform);
    wall
}

#[derive(Clone)]
pub struct RoomMaterialScheme {
    pub ceiling: Box<Shadable + Send + Sync>,
    pub floor: Box<Shadable + Send + Sync>,
    pub front: Box<Shadable + Send + Sync>,
    pub back: Box<Shadable + Send + Sync>,
    pub left: Box<Shadable + Send + Sync>,
    pub right: Box<Shadable + Send + Sync>,
}


pub fn create_room_from_material(size: f64, room_mat: RoomMaterialScheme) -> Box<SceneNode> {
    let mut interior_box = SceneNode::new();

    let floor = create_wall_from_material(size*1.01, room_mat.floor, rotation(Axis::X, -90.0) * translation(0.0, 0.0, -size/2.0));
    let ceiling = create_wall_from_material(size*1.01, room_mat.ceiling, rotation(Axis::X, 90.0) * translation(0.0, 0.0, -size/2.0));

    let front = create_wall_from_material(size*1.01, room_mat.front, translation(0.0, 0.0, -size/2.0));
    let back = create_wall_from_material(size*1.01, room_mat.back, rotation(Axis::X, 180.0) * translation(0.0, 0.0, -size/2.0));

    let left = create_wall_from_material(size*1.01, room_mat.left, rotation(Axis::Y, 90.0) * translation(0.0, 0.0, -size/2.0));
    let right = create_wall_from_material(size*1.01, room_mat.right, rotation(Axis::Y, -90.0) * translation(0.0, 0.0, -size/2.0));

    interior_box.add_child(Box::new(ceiling));
    interior_box.add_child(Box::new(floor));
    interior_box.add_child(Box::new(front));
    interior_box.add_child(Box::new(back));
    interior_box.add_child(Box::new(left));
    interior_box.add_child(Box::new(right));

    Box::new(interior_box)
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

    let floor = create_wall(size*1.01, room_color.floor, rotation(Axis::X, -90.0) * translation(0.0, 0.0, -size/2.0));
    let ceiling = create_wall(size*1.01, room_color.ceiling, rotation(Axis::X, 90.0) * translation(0.0, 0.0, -size/2.0));

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
    sphere.set_material(PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0));
    sphere.set_transform(transform);
    sphere
}

pub fn create_translucent_cube(size: f64, transform: DMat4, color: Color, refractive_index: f64) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(Cube::new(size)));
    sphere.set_material(TranslucentShader::new(color*1.0, refractive_index));
    sphere.set_transform(transform);
    sphere
}

pub fn create_translucent_sphere(size: f64, transform: DMat4, color: Color, refractive_index: f64) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(OneWay::new(Box::new(Sphere::new(size)))));
    sphere.set_material(TranslucentShader::new(color*1.0, refractive_index));
    sphere.set_transform(transform);
    sphere
}

pub fn create_mirror_sphere(size: f64, transform: DMat4, color: Color) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(OneWay::new(Box::new(Sphere::new(size)))));
    sphere.set_material(ReflectionShader::new(color*1.0));
    sphere.set_transform(transform);
    sphere
}

pub fn create_comp_sphere(size: f64, transform: DMat4, color: Color) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(OneWay::new(Box::new(Sphere::new(size)))));
    let phong = PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0);
    let reflect = ReflectionShader::new(Color::WHITE);
    let mut comp = CompositeShader::new();
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

pub fn xor_shape(transform: DMat4, primary: Box<Compositable + Send + Sync>, secondary: Box<Compositable + Send + Sync>) -> Box<XorShape> {
    let mut shape = XorShape::new(primary, secondary);
    shape.set_transform(transform);
    Box::new(shape)
}

pub fn and_shape(transform: DMat4, primary: Box<Compositable + Send + Sync>, secondary: Box<Compositable + Send + Sync>) -> Box<AndShape> {
    let mut shape = AndShape::new(primary, secondary);
    shape.set_transform(transform);
    Box::new(shape)
}

pub fn reuleaux_tetrahedron(transform: DMat4, size: f64) -> Box<MultiAndShape> {
    let radius = (2.0*size.powi(2)).sqrt();
    let spheres: Vec<Box<Compositable + Send + Sync>> = vec!(
        base_shape(translation(-size/2.0, -size/2.0, size/2.0), Sphere::from_radius(radius)),
        base_shape(translation(size/2.0, size/2.0, size/2.0), Sphere::from_radius(radius)),
        base_shape(translation(-size/2.0, size/2.0, -size/2.0), Sphere::from_radius(radius)),
        base_shape(translation(size/2.0, -size/2.0, -size/2.0), Sphere::from_radius(radius)),
    );
    let mut tetra = MultiAndShape::from_vec(spheres);
    /*
    let mut tetra = and_shape(
        DMat4::identity(),
        and_shape(
            DMat4::identity(),
            base_shape(translation(-size/2.0, -size/2.0, size/2.0), Sphere::from_radius(radius)),
            base_shape(translation(size/2.0, size/2.0, size/2.0), Sphere::from_radius(radius)),
        ),
        and_shape(
            DMat4::identity(),
            base_shape(translation(-size/2.0, size/2.0, -size/2.0), Sphere::from_radius(radius)),
            base_shape(translation(size/2.0, -size/2.0, -size/2.0), Sphere::from_radius(radius)),
        ),
    );
    */
    tetra.set_transform(transform);
    Box::new(tetra)
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
    let phong = PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0);
    let reflect = ReflectionShader::new(Color::WHITE);
    let mut comp = CompositeShader::new();
    comp.add_shader(0.8, phong);
    comp.add_shader(0.2, reflect);
    weird.set_material(comp);
    weird.set_transform(transform);
    Box::new(weird)
}

pub fn create_sphere(size: f64, transform: DMat4, color: Color) -> SceneNode {
    let mut sphere = SceneNode::new();
    sphere.set_primitive(Box::new(Sphere::new(size)));
    sphere.set_material(PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0));
    sphere.set_transform(transform);
    sphere
}
