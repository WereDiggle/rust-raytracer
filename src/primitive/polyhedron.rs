use super::*;

#[derive(Clone)]
pub struct Polyhedron {
    planes: Vec<Plane>,
}

impl Polyhedron {
    pub fn from_planes(planes: Vec<Plane>) -> Box<Polyhedron> {
        Box::new(Polyhedron{planes})
    }

    fn check_bounds(&self, intersect: Intersect, offset: usize) -> bool {
        for i in 1..self.planes.len() {
            let plane = &self.planes[(i+offset)%self.planes.len()];
            if (plane.origin - intersect.hit_point).dot(plane.normal) < 0.0 {
                return false;
            }
        }
        true
    }

}

impl Intersectable for Polyhedron {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        let mut ret_intersect: Option<Intersect> = None;
        for i in 0..self.planes.len() {
            let plane = &self.planes[i];
            if let Some(int) = plane.get_closest_intersect(ray) {
                if self.check_bounds(int, i) {
                    if let Some(ret_int) = ret_intersect {
                        if int.distance < ret_int.distance {
                            ret_intersect = Some(int);
                        }
                    }
                    else {
                        ret_intersect = Some(int);
                    }
                }                
            }
        }
        ret_intersect
    }
}