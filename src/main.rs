extern crate raytracer;
extern crate euler;

use raytracer::*;
use euler::*;

fn main() {
    let mut test_scene = Scene::new();
    test_scene.add_light(PointLight::new(dvec3!(0.0, 1000.0, 0.0), Color::new(1.0, 1.0, 1.0), 1.0, (1.0, 0.0, 0.0)));

    write_to_png(render(test_scene.clone(), 
                        ImageDimension{ width: 25, height: 25 },
                        CameraConfig{ origin: dvec3!(0.0, 0.0, 200.0),
                                      target: dvec3!(0.0, 0.0, 0.0),
                                      up: dvec3!(0.0, 1.0, 0.0),
                                      fov_y: 90.0}
                        ), 
                 "../output/test1");
}
