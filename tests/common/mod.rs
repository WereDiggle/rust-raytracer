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

