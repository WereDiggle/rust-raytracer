extern crate raytracer;
extern crate euler;

use raytracer::*;
use euler::*;
use std::f64::consts::PI;

fn main() {
    let light = PointLight::new(dvec3!(0.0, (200.0/2.0)*0.6, (200.0/2.0)*0.6), Color::new(1.0, 1.0, 1.0), 100000.0, (0.0, 0.0, 4.0*PI));
    let color = light.get_intensity_at_distance(200.0);
    println!("{:?}",color);
}
