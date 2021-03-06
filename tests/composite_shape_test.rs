extern crate raytracer;
extern crate euler;

use raytracer::*;
use raytracer::matrix::*;
use euler::*;
use std::f64::consts::PI;

#[test]
fn subtract_sphere() {
    let mut scene = Scene::new();
    let mut root = SceneNode::new();
    let sphere1 = Sphere::from_radius(50.0);
    let sphere2 = Sphere::from_radius(40.0);

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

#[test]
fn many_weirds() {
    let scene = build_scene(
        vec!(light1()),
        no_ambient(),
        Some(SkyBox::from_path(
            "assets/images/backgrounds/forest2.jpg",
            rotation(Axis::Y, 180.0),
        )),
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
    PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0)
}

fn clear_material() -> Box<TranslucentShader> {
    TranslucentShader::new(Color::WHITE, 1.52)
}

fn make_spikey_thing(size: f64) -> Box<SubtractShape> {
    subtract_shape(
        DMat4::identity(),
        subtract_shape(
            DMat4::identity(),
            subtract_shape(
                DMat4::identity(),
                base_shape(
                    DMat4::identity(),
                    sphere(size),
                ),
                base_shape(
                    translation(size, 0.0, 0.0),
                    sphere(size),
                ),
            ),
            base_shape(
                translation(0.0, size, 0.0),
                sphere(size),
            ),
        ),
        base_shape(
            translation(0.0, 0.0, size),
            sphere(size),
        ),
    )
}

fn make_weird_cube(size: f64) -> Box<SubtractShape> {
    subtract_shape(
        DMat4::identity(),
        base_shape(
            DMat4::identity(),
            cube(size*2.0),
        ),
        base_shape(
            DMat4::identity(),
            sphere(size),
        ),
    )
}

fn subracted_sphere(size: f64) -> Box<SubtractShape> {
    subtract_shape(
        DMat4::identity(),
        base_shape(
            DMat4::identity(),
            sphere(size),
        ),
        base_shape(
            DMat4::identity(),
            cube(size*0.7),
        ),
    )
}

fn double_subracted_sphere(size: f64) -> Box<SubtractShape> {
    subtract_shape(
        DMat4::identity(),
        base_shape(
            DMat4::identity(),
            sphere(size),
        ),
        make_weird_cube(size*0.7)
    )
}

#[test]
fn subtraction_room() {
    let sphere_size: f64 = 80.0;
    let scene = build_scene(
        vec!(light1(), light2()),
        no_ambient(),
        None,
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
                    make_spikey_thing(sphere_size),
                    vec!(),
                ),
                geometry_node(
                    translation(100.0, 0.0, 40.0)*rotation(Axis::Y, 45.0),
                    clear_material(),
                    make_weird_cube(sphere_size),
                    vec!(),
                ),
            ),
        ),
    );

    let mut render_config = RenderConfig::default();
    render_config.anti_alias = true;
    let image = render_with_config(scene, image(1920, 1080), camera([0.0, 0.0, 350.0], [0.0, -100.0, -350.0]), render_config);
    write_to_png( image, "output/subtraction_room");
}

#[test]
fn subtraction_outside1() {
    let sphere_size: f64 = 80.0;
    let scene = build_scene(
        vec!(light1(), light2()),
        no_ambient(),
        Some(SkyBox::from_path(
            "assets/images/backgrounds/building.jpg",
            rotation(Axis::Y, 0.0),
        )),
        scene_node(
            DMat4::identity(),
            vec!(
                geometry_node(
                    translation(-150.0, 0.0, 40.0)*rotation(Axis::Z, 60.0),
                    clear_material(),
                    make_spikey_thing(sphere_size),
                    vec!(),
                ),
                geometry_node(
                    translation(100.0, 0.0, 40.0)*rotation(Axis::Y, 45.0),
                    clear_material(),
                    make_weird_cube(sphere_size),
                    vec!(),
                ),
            ),
        ),
    );

    let mut render_config = RenderConfig::default();
    render_config.anti_alias = true;
    let image = render_with_config(scene, image(5000, 5000), camera([0.0, 0.0, 350.0], [0.0, -100.0, -350.0]), render_config);
    write_to_png( image, "output/subtraction_outside");
}

#[test]
fn subtraction_outside3() {
    let sphere_size: f64 = 100.0;
    let scene = build_scene(
        vec!(light1(), light2()),
        no_ambient(),
        Some(SkyBox::from_path(
            "assets/images/backgrounds/building.jpg",
            rotation(Axis::Y, 11.0)
        )),
        scene_node(
            DMat4::identity(),
            vec!(
                geometry_node(
                    translation(0.0, 0.0, 40.0)*rotation(Axis::Y, 45.0),
                    clear_material(),
                    subracted_sphere(sphere_size),
                    vec!(),
                ),
            ),
        ),
    );

    let mut render_config = RenderConfig::default();
    render_config.anti_alias = true;
    let image = render_with_config(scene, image(5000, 5000), camera([0.0, 0.0, 350.0], [0.0, -100.0, -350.0]), render_config);
    write_to_png( image, "output/subtraction_outside3");
}

fn make_round_cube(size: f64) -> Box<AndShape> {
    and_shape(
        DMat4::identity(),
        base_shape(
            DMat4::identity(),
            cube(size*1.5),
        ),
        base_shape(
            DMat4::identity(),
            sphere(size),
        ),
    )
}

fn shiney_material(color: Color) -> Box<CompositeShader> {
    let mut comp_material = CompositeShader::new();
    let phong = PhongShader::new(color*0.5, color*0.5, color*0.01, 4.0);
    let reflect = ReflectionShader::new(Color::WHITE);
    comp_material.add_shader(0.8, phong);
    comp_material.add_shader(0.2, reflect);
    comp_material
}

#[test]
fn rounded_cube() {
    let sphere_size: f64 = 100.0;
    let scene = build_scene(
        vec!(light1(), light2()),
        no_ambient(),
        Some(SkyBox::from_path(
            "assets/images/backgrounds/building.jpg",
            rotation(Axis::Y, 0.0),
        )),
        scene_node(
            DMat4::identity(),
            vec!(
                geometry_node(
                    translation(0.0, 0.0, 40.0)*rotation(Axis::Y, 45.0)*rotation(Axis::Z, 15.0),
                    shiney_material(Color::MAROON),
                    make_round_cube(sphere_size),
                    vec!(),
                ),
            ),
        ),
    );

    let mut render_config = RenderConfig::default();
    render_config.anti_alias = true;
    let image = render_with_config(scene, image(1920, 1080), camera([0.0, 0.0, 350.0], [0.0, -100.0, -350.0]), render_config);
    write_to_png( image, "output/rounded_cube");
}

fn make_xor_sphere(size: f64) -> Box<XorShape> {
    xor_shape(
        DMat4::identity(),
        base_shape(
            translation(size/4.0, 0.0, 0.0),
            sphere(size),
        ),
        base_shape(
            translation(-size/4.0, 0.0, 0.0),
            sphere(size),
        ),
    )
}

#[test]
fn xor_test() {
    let sphere_size: f64 = 100.0;
    let scene = build_scene(
        vec!(light1(), light2()),
        no_ambient(),
        Some(SkyBox::from_path(
            "assets/images/backgrounds/building.jpg",
            rotation(Axis::Y, 77.0),
        )),
        scene_node(
            DMat4::identity(),
            vec!(
                geometry_node(
                    translation(0.0, 0.0, 40.0),
                    clear_material(),
                    make_xor_sphere(sphere_size),
                    vec!(),
                ),
            ),
        ),
    );

    let mut render_config = RenderConfig::default();
    render_config.anti_alias = true;
    let image = render_with_config(scene, image(1920, 1080), camera([0.0, 0.0, 350.0], [0.0, -100.0, -350.0]), render_config);
    write_to_png( image, "output/xor_test");
}

#[test]
fn subtraction_outside4() {
    let sphere_size: f64 = 100.0;
    let scene = build_scene(
        vec!(light1(), light2()),
        no_ambient(),
        Some(SkyBox::from_path(
            "assets/images/backgrounds/building.jpg",
            rotation(Axis::Y, 130.0),
        )),
        scene_node(
            DMat4::identity(),
            vec!(
                geometry_node(
                    translation(10.0, 0.0, 40.0)*rotation(Axis::Y, 45.0),
                    clear_material(),
                    subtract_shape(
                        DMat4::identity(),
                        make_round_cube(sphere_size),
                        make_xor_sphere(sphere_size*0.5),
                    ),
                    vec!(),
                ),
            ),
        ),
    );

    let mut render_config = RenderConfig::default();
    render_config.anti_alias = true;
    let image = render_with_config(scene, image(5000, 5000), camera([0.0, 0.0, 350.0], [0.0, -100.0, -350.0]), render_config);
    write_to_png( image, "output/subtraction_outside4");
}

#[test]
fn reuleaux() {
    let size: f64 = 100.0;
    let scene = build_scene(
        vec!(light1(), light2()),
        no_ambient(),
        Some(SkyBox::from_path(
            "assets/images/backgrounds/building.jpg",
            rotation(Axis::Y, 200.0),
        )),
        scene_node(
            DMat4::identity(),
            vec!(
                geometry_node(
                    translation(10.0, 0.0, 40.0),
                    default_material(Color::RED),
                    reuleaux_tetrahedron(
                        DMat4::identity(),
                        size,
                    ),
                    vec!(),
                ),
            ),
        ),
    );

    let mut render_config = RenderConfig::default();
    render_config.anti_alias = true;
    let image = render_with_config(scene, image(512, 512), camera([-200.0, 350.0, 50.0], [10.0, 0.0, 40.0]), render_config);
    write_to_png( image, "output/reuleaux");
}