use euler::{dmat4, DMat4, DVec2, DVec3, dvec3, dvec4, dvec2};
use shader::Shadable;
use color::*;
use snowflake::ProcessUniqueId;

pub mod matrix;
pub mod ray;
pub mod surface_coord;
pub mod intersect;

pub use self::ray::Ray;
pub use self::surface_coord::SurfaceCoord;
pub use self::intersect::{NodeIntersect, Intersect};

pub trait Intersectable: IntersectableClone {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect>;

    // hit_point needs to be in model space
    //fn surface_normal(&self, hit_point: DVec3) -> DVec3;

    //fn surface_coords(&self, hit_point: DVec4) -> DVec2;

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let mut ret_intersects: Vec<Intersect> = Vec::new(); 
        if let Some(intersect) = self.get_closest_intersect(ray) {
            ret_intersects.push(intersect);
        }
        ret_intersects
    }
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