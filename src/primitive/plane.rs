use super::*;

#[derive(Clone)]
pub struct RectangularPlane {
    pub width: f64,
    pub height: f64,
}

impl RectangularPlane {
    pub fn new(width: f64, height: f64) -> RectangularPlane {
        RectangularPlane{width, height}
    }
}

impl Intersectable for RectangularPlane {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        // Get point on the plane
        let surface_normal = dvec3!(0.0, 0.0, 1.0);
        let hit_distance = ray.origin.dot(surface_normal) / ray.direction.dot(surface_normal) * -1.0;
        let hit_point = ray.point_at_distance(hit_distance);

        // Check point against bounds
        let horizontal_bound = self.width/2.0;
        let vertical_bound = self.height/2.0;
        if hit_point.x < -horizontal_bound || hit_point.x > horizontal_bound ||
            hit_point.y < -vertical_bound || hit_point.y > vertical_bound {
            return None;
        }

        if hit_distance >= Ray::MIN_DISTANCE {
            let u = (hit_point.x/horizontal_bound + 1.0)/2.0;
            let v = (hit_point.y/vertical_bound + 1.0)/2.0;
            let surface_coord = SurfaceCoord::new(u, v);
            Some(Intersect::new(ray, hit_distance, hit_point, surface_normal, surface_coord))
        }
        else {
            None
        }
    }

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let mut ret_intersects: Vec<Intersect> = Vec::new(); 
        if let Some(intersect) = self.get_closest_intersect(ray) {
            ret_intersects.push(intersect);
        }
        ret_intersects
    }
}