use super::*;

#[derive(Clone)]
pub struct Polyhedron {
    planes: Vec<Plane>,
}

impl Polyhedron {
    pub fn from_planes(planes: Vec<Plane>) -> Box<Polyhedron> {
        Box::new(Polyhedron{planes})
    }

    pub fn octahedron(size: f64) -> Box<Polyhedron> {
        use euler::dvec2;
        let half_size = size / 2.0;
        let n = (
            dvec3!(-1.0,1.0,1.0).normalize(),
            dvec3!(1.0,1.0,1.0).normalize(),
            dvec3!(-1.0,1.0,-1.0).normalize(),
            dvec3!(1.0,1.0,-1.0).normalize(),

            dvec3!(-1.0,-1.0,1.0).normalize(),
            dvec3!(1.0,-1.0,1.0).normalize(),
            dvec3!(-1.0,-1.0,-1.0).normalize(),
            dvec3!(1.0,-1.0,-1.0).normalize(),
        );
        let up = dvec3!(0.0, 1.0, 0.0);
        let down = dvec3!(0.0, -1.0, 0.0);
        let o = (
            dvec3!(-half_size,0.0,0.0),
            dvec3!(0.0,0.0,half_size),
            dvec3!(0.0,0.0,-half_size),
            dvec3!(half_size,0.0,0.0),

            dvec3!(0.0,0.0,half_size),
            dvec3!(half_size,0.0,0.0),
            dvec3!(-half_size,0.0,0.0),
            dvec3!(0.0,0.0,-half_size),
        );

        // TODO: figure out length to surface scale
        let surface_scale = dvec2!(size/(2.0 as f64).sqrt(), size/(2.0 as f64).sqrt());
        Polyhedron::from_planes(vec!(
            *Plane::with_surface_scale(o.0, n.0, up, surface_scale),
            *Plane::with_surface_scale(o.1, n.1, up, surface_scale),
            *Plane::with_surface_scale(o.2, n.2, up, surface_scale),
            *Plane::with_surface_scale(o.3, n.3, up, surface_scale),

            *Plane::with_surface_scale(o.4, n.4, down, surface_scale),
            *Plane::with_surface_scale(o.5, n.5, down, surface_scale),
            *Plane::with_surface_scale(o.6, n.6, down, surface_scale),
            *Plane::with_surface_scale(o.7, n.7, down, surface_scale),
        ))
    }

    pub fn deltohedron(size: f64) -> Box<Polyhedron> {
        use euler::dvec2;
        let half_size = size / 2.0;
        let n = (
            dvec3!(-1.0,1.0,1.0).normalize(),
            dvec3!(1.0,1.0,1.0).normalize(),
            dvec3!(-1.0,1.0,-1.0).normalize(),
            dvec3!(1.0,1.0,-1.0).normalize(),

            dvec3!(-1.0,-1.0,1.0).normalize(),
            dvec3!(1.0,-1.0,1.0).normalize(),
            dvec3!(-1.0,-1.0,-1.0).normalize(),
            dvec3!(1.0,-1.0,-1.0).normalize(),
        );
        let up = dvec3!(0.0, 1.0, 0.0);
        let down = dvec3!(0.0, -1.0, 0.0);
        let o = (
            dvec3!(-half_size,0.0,0.0),
            dvec3!(0.0,0.0,half_size),
            dvec3!(0.0,0.0,-half_size),
            dvec3!(half_size,0.0,0.0),

            dvec3!(0.0,0.0,half_size),
            dvec3!(half_size,0.0,0.0),
            dvec3!(-half_size,0.0,0.0),
            dvec3!(0.0,0.0,-half_size),
        );

        // TODO: figure out length to surface scale
        let surface_scale = dvec2!(size/(2.0 as f64).sqrt(), size/(2.0 as f64).sqrt());
        Polyhedron::from_planes(vec!(
            *Plane::with_surface_scale(o.0, n.0, up, surface_scale),
            *Plane::with_surface_scale(o.1, n.1, up, surface_scale),
            *Plane::with_surface_scale(o.2, n.2, up, surface_scale),
            *Plane::with_surface_scale(o.3, n.3, up, surface_scale),

            *Plane::with_surface_scale(o.4, n.4, down, surface_scale),
            *Plane::with_surface_scale(o.5, n.5, down, surface_scale),
            *Plane::with_surface_scale(o.6, n.6, down, surface_scale),
            *Plane::with_surface_scale(o.7, n.7, down, surface_scale),
        ))
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

    fn modify_surface_coord<'a>(&self, intersect: &'a mut Intersect, offset: usize) -> &'a mut Intersect {
        let old_coord = intersect.surface_coord.get_coord();
        intersect.surface_coord = SurfaceCoord::new((offset as f64 + old_coord.0)/(self.planes.len() as f64), old_coord.1);
        intersect
    }

}

impl Intersectable for Polyhedron {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        let mut ret_intersect: Option<Intersect> = None;
        for i in 0..self.planes.len() {
            let plane = &self.planes[i];
            if let Some(mut int) = plane.get_closest_intersect(ray) {
                if self.check_bounds(int, i) {
                    if let Some(ret_int) = ret_intersect {
                        if int.distance < ret_int.distance {
                            ret_intersect = Some(*self.modify_surface_coord(&mut int, i));
                        }
                    }
                    else {
                        ret_intersect = Some(*self.modify_surface_coord(&mut int, i));
                    }
                }                
            }
        }
        ret_intersect
    }
}