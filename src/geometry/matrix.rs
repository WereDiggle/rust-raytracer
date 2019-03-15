use super::*;

pub enum Axis {
    X,
    Y,
    Z,
}

pub fn transform_point(matrix: DMat4, point: DVec3) -> DVec3 {
    (matrix * dvec4!(point, 1.0)).xyz()
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
    let side = forward.cross(up).normalize();
    let up = side.cross(forward).normalize();

    // The transformation matrix
    dmat4!(
        side.x, side.y, side.z, 0.0,
        up.x, up.y, up.z, 0.0,
        forward.x, forward.y, forward.z, 0,
        0, 0, 0, 1,
    )
}