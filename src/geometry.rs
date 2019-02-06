use euler::{dmat4, DMat4, DVec3, dvec4};
use scene::SceneNode;

pub mod matrix {
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
        // TODO: maybe there's a way to use the stored inverse matrix instead of calculating it
        (matrix.inverse().transpose() * dvec4!(normal, 0.0)).xyz().normalize()
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
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub const MIN_DISTANCE: f64 = 0.001;
    pub fn new(origin: DVec3, direction: DVec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn new_destination(origin: DVec3, destination: DVec3) -> Ray {
        Ray { origin, direction: (destination - origin).normalize()}
    }

    pub fn point_at_distance(&self, distance: f64) -> DVec3 {
        self.origin + (distance * self.direction)
    }

    pub fn transform(&self, matrix: DMat4) -> Ray {
        Ray { origin: (matrix * dvec4!(self.origin, 1.0)).xyz(),
              direction: (matrix * dvec4!(self.direction, 0.0)).xyz().normalize(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Intersect<'a> {
    pub node: &'a SceneNode,
    pub ray: Ray,
    pub distance: f64, 
    pub hit_point: DVec3,
    pub surface_normal: DVec3,
}

impl<'a> Intersect<'a> {
    pub fn new(node: &'a SceneNode, ray: Ray, distance: f64, hit_point: DVec3, surface_normal: DVec3) -> Intersect<'a> {
        Intersect {node, ray, distance, hit_point, surface_normal}
    }

    pub fn transform(&mut self, matrix: DMat4) -> Intersect<'a> {
        let ray = self.ray.transform(matrix);
        let hit_point = matrix::transform_point(matrix, self.hit_point);
        let distance = (hit_point - ray.origin).length();
        let surface_normal = matrix::transform_normal(matrix, self.surface_normal);
        Intersect {
            node: self.node,
            ray,
            distance,
            hit_point,
            surface_normal,
        }
    }
}

pub trait Intersectable: IntersectableClone {
    fn check_intersect(&self, ray: Ray) -> Option<f64>;

    // hit_point needs to be in model space
    fn surface_normal(&self, hit_point: DVec3) -> DVec3;

    //fn surface_coords(&self, hit_point: DVec4) -> DVec2;
}

pub trait IntersectableClone {
    fn clone_box(&self) -> Box<Intersectable + Send + Sync>;
}

impl<T> IntersectableClone for T
where
    T: 'static + Intersectable + Send + Sync + Clone
{
    fn clone_box(&self) -> Box<Intersectable + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Intersectable + Send + Sync> {
    fn clone(&self) -> Box<Intersectable + Send + Sync> {
        self.clone_box()
    }
}
