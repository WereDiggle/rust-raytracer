use euler::{dvec3, DVec3, DMat4};
use geometry::{Intersectable, Ray, matrix::*};

pub struct SubtractShape {
    minuend: Box<Intersectable + Send + Sync>,
    subtrahend: Box<Intersectable + Send + Sync>,
}

impl SubtractShape {
    pub fn subtract_shape(minuend: Box<Intersectable + Send + Sync>, subtrahend: Box<Intersectable + Send + Sync>) -> SubtractShape {
        SubtractShape {
            minuend,
            subtrahend,
        }
    }
}
