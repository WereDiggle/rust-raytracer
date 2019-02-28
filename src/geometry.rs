use euler::{dmat4, DMat4, DVec2, DVec3, dvec4, dvec2};
use shader::Shadable;
use color::*;
use snowflake::ProcessUniqueId;

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
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
    depth: u32,
    contribution: Color,
}

impl Ray {
    pub const MIN_DISTANCE: f64 = 0.001;
    pub const MIN_CONTRIBUTION: f64 = 0.003;
    pub fn new(origin: DVec3, direction: DVec3, depth: u32) -> Ray {
        Ray { origin, direction, depth, contribution: Color::from_f64(1.0) }
    }

    pub fn from_destination(origin: DVec3, destination: DVec3, depth: u32) -> Ray {
        Ray { origin, direction: (destination - origin).normalize(), depth, contribution: Color::from_f64(1.0) }
    }

    pub fn contributes(&self, percentage: Color) -> Ray {
        let mut new_ray = self.clone();
        new_ray.contribution *= percentage;
        new_ray
    }

    pub fn get_contribution(&self) -> f64 {
        self.contribution.red +
        self.contribution.green +
        self.contribution.blue
    }

    pub fn get_depth(&self) -> u32 {
        self.depth
    }

    pub fn reflect_off(&self, hit_point: DVec3, surface_normal: DVec3) -> Ray {
        assert!(surface_normal.length() - 1.0 < 0.000001);
        // math
        let dot = self.direction.dot(surface_normal);
        assert!(dot <= 0.0);

        let reflection_direction = (self.direction - 2.0 * dot * surface_normal).normalize();

        assert!(self.depth > 0);
        Ray {
            origin: hit_point,
            direction: reflection_direction,
            depth: self.depth - 1,
            contribution: self.contribution,
        }
    }

    pub fn transmit_through(&self, hit_point: DVec3, surface_normal: DVec3, refractive_index: f64) -> Ray {
        assert!(surface_normal.length() - 1.0 < 0.000001);
        assert!(self.depth > 0);

        let mut incident_cos = self.direction.dot(surface_normal);
        let mut refractive_index = refractive_index;

        let mut normal_sign = -1.0;
        if incident_cos < 0.0 {
            incident_cos = -incident_cos;
            refractive_index = 1.0/refractive_index;
            normal_sign = 1.0; 
        }

        let refraction_factor = 1.0 - refractive_index*refractive_index * (1.0 - incident_cos*incident_cos);

        if refraction_factor < 0.0 {
            self.reflect_off(hit_point, surface_normal * normal_sign)
        }
        else {
            let refraction_direction = refractive_index * self.direction + (refractive_index*incident_cos-refraction_factor.sqrt()) * normal_sign * surface_normal;
            Ray {
                origin: hit_point,
                direction: refraction_direction,
                depth: self.depth - 1,
                contribution: self.contribution,
            }
        }
    }

    pub fn point_at_distance(&self, distance: f64) -> DVec3 {
        self.origin + (distance * self.direction)
    }

    pub fn transform(&self, matrix: DMat4) -> Ray {
        Ray { 
            origin: (matrix * dvec4!(self.origin, 1.0)).xyz(),
            direction: (matrix * dvec4!(self.direction, 0.0)).xyz().normalize(),
            depth: self.depth,
            contribution: self.contribution,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SurfaceCoord {
    coord: DVec2,
}

impl SurfaceCoord {
    pub fn new(u: f64, v: f64) -> SurfaceCoord {
        SurfaceCoord {coord: dvec2!(u, v)}
    }

    pub fn get_u(&self) -> f64 {self.coord.x}
    pub fn get_v(&self) -> f64 {self.coord.y}
    pub fn get_coord(&self) -> (f64, f64) {(self.coord.x, self.coord.y)}
}

#[derive(Clone, Copy)]
pub struct Intersect {
    pub ray: Ray,
    pub distance: f64, 
    pub hit_point: DVec3,
    pub surface_normal: DVec3,
    pub surface_coord: SurfaceCoord,
}

impl Intersect {
    pub fn new(ray: Ray, distance: f64, hit_point: DVec3, surface_normal: DVec3, surface_coord: SurfaceCoord) -> Intersect {
        Intersect{ ray, distance, hit_point, surface_normal, surface_coord}
    }

    pub fn transform(&self, matrix: DMat4) -> Intersect {
        let ray = self.ray.transform(matrix);
        let hit_point = matrix::transform_point(matrix, self.hit_point);
        let distance = (hit_point - ray.origin).length();
        let surface_normal = matrix::transform_normal(matrix, self.surface_normal);
        Intersect {
            ray,
            distance,
            hit_point,
            surface_normal,
            surface_coord: self.surface_coord,
        }
    }

    pub fn contributes(&self, percentage: Color) -> Intersect {
        Intersect {
            ray: self.ray.contributes(percentage),
            distance: self.distance,
            hit_point: self.hit_point,
            surface_normal: self.surface_normal,
            surface_coord: self.surface_coord,
        }
    }

    pub fn invert_normal(&self) -> Intersect {
        Intersect {
            ray: self.ray,
            distance: self.distance,
            hit_point: self.hit_point,
            surface_normal: self.surface_normal * -1.0,
            surface_coord: self.surface_coord,
        }
    }
}

#[derive(Clone, Copy)]
pub struct NodeIntersect<'a> {
    pub hit_id: ProcessUniqueId,
    pub shader: &'a (Shadable + Send + Sync + 'a),
    pub intersect: Intersect,
}

impl<'a> NodeIntersect<'a> {
    pub fn new(hit_id: ProcessUniqueId, shader: &'a (Shadable + Send + Sync + 'a), intersect: Intersect) -> NodeIntersect<'a> {
        NodeIntersect {hit_id, shader, intersect}
    }

    pub fn transform(&self, matrix: DMat4) -> NodeIntersect<'a> {
        NodeIntersect {
            hit_id: self.hit_id,
            shader: self.shader,
            intersect: self.intersect.transform(matrix),
        }
    }

    pub fn get_distance(&self) -> f64 {
        self.intersect.distance
    }

    pub fn get_hit_point(&self) -> DVec3 {
        self.intersect.hit_point
    }

    pub fn get_surface_normal(&self) -> DVec3 {
        self.intersect.surface_normal
    }

    pub fn get_ray(&self) -> Ray {
        self.intersect.ray
    }
}

pub trait Intersectable: IntersectableClone {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect>;

    // hit_point needs to be in model space
    //fn surface_normal(&self, hit_point: DVec3) -> DVec3;

    //fn surface_coords(&self, hit_point: DVec4) -> DVec2;

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect>;
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

pub trait Transformable: TransformableClone {

    fn set_transform(&mut self, trans: DMat4);
    fn get_transform(&self) -> DMat4;
    fn get_inverse_transform(&self) -> DMat4;

    fn transform(&mut self, trans: DMat4) {
        let matrix = self.get_transform();
        self.set_transform(trans * matrix);
    }

}

pub trait TransformableClone {
    fn clone_box(&self) -> Box<Transformable + Send + Sync>;
}

impl<T> TransformableClone for T
where
    T: 'static + Transformable + Send + Sync + Clone
{
    fn clone_box(&self) -> Box<Transformable + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Transformable + Send + Sync> {
    fn clone(&self) -> Box<Transformable + Send + Sync> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct TransformComponent {
    trans: DMat4,
    inv_trans: DMat4,
}

impl TransformComponent {

    pub fn new(trans: DMat4) -> TransformComponent {
        TransformComponent{trans, inv_trans: trans.inverse()}
    }
}

impl Transformable for TransformComponent {

    fn set_transform(&mut self, trans: DMat4) {
        self.trans = trans;
        self.inv_trans = trans.inverse();
    }

    fn get_transform(&self) -> DMat4 {
        self.trans
    }

    fn get_inverse_transform(&self) -> DMat4 {
        self.inv_trans
    }
}