use super::*;

#[derive(Clone)]
pub struct Sphere {
    pub radius: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Sphere {
        Sphere {radius}
    }

    fn get_surface_coord(hit_point: DVec3) -> SurfaceCoord {
        let hit_point = hit_point.normalize();
        let azimuth = hit_point.z.atan2(hit_point.x);
        let elevation = hit_point.y.asin();

        let u = (azimuth/2.0)/PI + 0.5;
        let v = elevation/PI + 0.5;
        assert!(u >= 0.0 && u <= 1.0);
        assert!(v >= 0.0 && v <= 1.0);

        SurfaceCoord::new(u, v)
    }

    fn two_intersects(&self, ray: Ray) -> (Option<Intersect>, Option<Intersect>) {
        let l = -1.0 * ray.origin;
        let adj = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return (None, None);
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < Ray::MIN_DISTANCE && t1 < Ray::MIN_DISTANCE {
            return (None, None);
        }
        let mut intersects: (Option<Intersect>, Option<Intersect>) = (None, None);

        if t0 >= Ray::MIN_DISTANCE {
            let hit_point = ray.point_at_distance(t0);
            let surface_coord = Sphere::get_surface_coord(hit_point);
            intersects.0 = Some(Intersect::new(ray, t0, hit_point, hit_point.normalize(), surface_coord));
        } 

        if t1 >= Ray::MIN_DISTANCE {
            let hit_point = ray.point_at_distance(t1);
            let surface_coord = Sphere::get_surface_coord(hit_point);
            intersects.1 = Some(Intersect::new(ray, t1, hit_point, hit_point.normalize(), surface_coord));
        }

        intersects
    }
}

impl Intersectable for Sphere {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        let intersects = self.two_intersects(ray);
        if let Some(intersect0) = intersects.0 {
            if let Some(intersect1) = intersects.1 {
                if intersect0.distance < intersect1.distance {
                    Some(intersect0)
                }
                else {
                    Some(intersect1)
                }
            }
            else {
                intersects.0
            }
        }
        else {
            intersects.1
        }
    }

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let intersects = self.two_intersects(ray);
        let mut ret_intersects: Vec<Intersect> = Vec::new();
        if let Some(intersect) = intersects.0 {
            ret_intersects.push(intersect);
        }
        if let Some(intersect) = intersects.1 {
            ret_intersects.push(intersect);
        }
        ret_intersects
    }
}
