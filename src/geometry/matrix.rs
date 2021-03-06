use super::*;
use std::f64::{INFINITY, NEG_INFINITY};

pub const UP: DVec3 = DVec3{x: 0.0, y: 1.0, z: 0.0};
pub const DOWN: DVec3 = DVec3{x: 0.0, y: -1.0, z: 0.0};
pub const LEFT: DVec3 = DVec3{x: -1.0, y: 0.0, z: 0.0};
pub const RIGHT: DVec3 = DVec3{x: 1.0, y: 0.0, z: 0.0};
pub const BACK: DVec3 = DVec3{x: 0.0, y: 0.0, z: -1.0};
pub const FRONT: DVec3 = DVec3{x: 0.0, y: 0.0, z: 1.0};

pub const INF: DVec3 = DVec3{x: INFINITY, y: INFINITY, z: INFINITY};
pub const NEG_INF: DVec3 = DVec3{x: NEG_INFINITY, y: NEG_INFINITY, z: NEG_INFINITY};

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub fn max_bound(a: DVec3, b: DVec3) -> DVec3 {
    dvec3!(
        a.x.max(b.x),
        a.y.max(b.y),
        a.z.max(b.z)
    )
}

pub fn min_bound(a: DVec3, b: DVec3) -> DVec3 {
    dvec3!(
        a.x.min(b.x),
        a.y.min(b.y),
        a.z.min(b.z)
    )
}

impl Axis {
    pub fn value(&self, v: DVec3) -> f64 {
        match self {
            Axis::X => v.x,
            Axis::Y => v.y,
            Axis::Z => v.z,
        }
    }
}

pub fn transform_point(matrix: DMat4, point: DVec3) -> DVec3 {
    (matrix * dvec4!(point, 1.0)).xyz()
}

pub fn transform_vector(matrix: DMat4, vector: DVec3) -> DVec3 {
    (matrix * dvec4!(vector, 0.0)).xyz()
}

pub fn transform_normal(matrix: DMat4, normal: DVec3) -> DVec3 {
    (matrix.inverse().transpose() * dvec4!(normal, 0.0)).xyz().normalize()
}

pub fn transform_normal_with_inverse(inv_matrix: DMat4, normal: DVec3) -> DVec3 {
    (inv_matrix.transpose() * dvec4!(normal, 0.0)).xyz().normalize()
}

pub fn rotation(axis: Axis, degree: f64) -> DMat4 {
    let t = degree.to_radians();
    let sin_t = t.sin();
    let cos_t = t.cos();
    match axis {
        Axis::X => dmat4!(1.0,    0.0,   0.0, 0.0,
                            0.0,  cos_t, sin_t, 0.0,
                            0.0, -sin_t, cos_t, 0.0,
                            0.0,    0.0,   0.0, 1.0,),

        Axis::Y => dmat4!(cos_t, 0.0, -sin_t, 0.0,
                            0.0, 1.0,    0.0, 0.0,
                            sin_t, 0.0,  cos_t, 0.0,
                            0.0, 0.0,    0.0, 1.0,),

        Axis::Z => dmat4!( cos_t, sin_t, 0.0, 0.0,
                            -sin_t, cos_t, 0.0, 0.0,
                                0.0,   0.0, 1.0, 0.0,
                                0.0,   0.0, 0.0, 1.0,),
    }
}

pub fn translation(x: f64, y: f64, z: f64) -> DMat4 {
    dmat4!( 1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
                x,   y,   z, 1.0,)
}

pub fn scaling(x: f64, y: f64, z: f64) -> DMat4 {
    dmat4!(   x, 0.0, 0.0, 0.0,
            0.0,   y, 0.0, 0.0,
            0.0, 0.0,   z, 0.0,
            0.0, 0.0, 0.0, 1.0,)
}

pub fn reflection(axis: Axis) -> DMat4 {
    let (mut x, mut y, mut z) = (1.0, 1.0, 1.0);
    match axis {
        Axis::X => x = -x,
        Axis::Y => y = -y,
        Axis::Z => z = -z,
    }
    dmat4!(   x, 0.0, 0.0, 0.0,
            0.0,   y, 0.0, 0.0,
            0.0, 0.0,   z, 0.0,
            0.0, 0.0, 0.0, 1.0,)
}

pub fn basis(forward: DVec3, up: DVec3) -> DMat4 {

    // Set up the Basis directions
    // TODO: remove this hacky hack
    let up = if up == forward {dvec3!(0.0, 0.0, -1.0)} else {up};
    let side = forward.cross(up).normalize();
    assert!(side.length() - 1.0 < 0.0001, "side: {}", side);
    let up = side.cross(forward).normalize();

    // The transformation matrix
    dmat4!(
        side.x, side.y, side.z, 0.0,
        up.x, up.y, up.z, 0.0,
        forward.x, forward.y, forward.z, 0,
        0, 0, 0, 1,
    )
}