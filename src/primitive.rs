use euler::{dvec3, DVec3, DMat4};
use geometry::{SurfaceCoord, Intersect, Intersectable, Ray, matrix::*};
use std::f64::consts::PI;

pub mod cube;
pub mod sphere;
pub mod plane;

pub use self::cube::Cube;
pub use self::sphere::Sphere;
pub use self::plane::RectangularPlane;

#[derive(Clone)]
pub struct OneWay {
    primitive: Box<Intersectable + Send + Sync>,
}

impl OneWay {
    pub fn new(primitive: Box<Intersectable + Send + Sync>) -> OneWay {
        OneWay {
            primitive,
        }
    }
}

impl Intersectable for OneWay {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        if let Some(intersect) = self.primitive.get_closest_intersect(ray) {
            if ray.direction.dot(intersect.surface_normal) < 0.0 {
                return Some(intersect);
            }
        }
        None
    }

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let intersects = self.primitive.get_all_intersects(ray);
        intersects.into_iter().filter(|intersect| ray.direction.dot(intersect.surface_normal) < 0.0).collect()
    }
}

#[derive(Clone)]
pub struct Inverted {
    primitive: Box<Intersectable + Send + Sync>,
}

impl Inverted {
    pub fn new(primitive: Box<Intersectable + Send + Sync>) -> Inverted {
        Inverted {
            primitive,
        }
    }
}

impl Intersectable for Inverted {
    // TODO: we really need to call get_all_intersects here because the closest inverted intersect
    // probably won't be the closest intersect
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        if let Some(mut intersect) = self.primitive.get_closest_intersect(ray) {
            if ray.direction.dot(intersect.surface_normal) >= 0.0 {
                intersect.surface_normal *= -1.0;
                return Some(intersect);
            }
        }
        None
    }

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let mut intersects = self.primitive.get_all_intersects(ray);
        for intersect in intersects.iter_mut() {
            intersect.surface_normal *= -1.0;
        }
        intersects
    }
}
