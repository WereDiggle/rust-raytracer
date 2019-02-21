use euler::{dvec3, DVec3, DMat4};
use geometry::{Intersect, Intersectable, Ray, matrix::*};

#[derive(Clone)]
pub struct OneWay {
    primitive: Box<Intersectable + Send + Sync>,
}

impl OneWay {
    pub fn new(primitive: Box<Intersectable + Send + Sync>) -> OneWay {
        OneWay {
            primitive,
        }
    }
}

impl Intersectable for OneWay {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        if let Some(intersect) = self.primitive.get_closest_intersect(ray) {
            if ray.direction.dot(intersect.surface_normal) < 0.0 {
                return Some(intersect);
            }
        }
        None
    }

/*
    fn surface_normal(&self, hit_point: DVec3) -> DVec3 {
        self.primitive.surface_normal(hit_point)
    }
    */

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let intersects = self.primitive.get_all_intersects(ray);
        intersects.into_iter().filter(|intersect| ray.direction.dot(intersect.surface_normal) < 0.0).collect()
    }
}

#[derive(Clone)]
pub struct Inverted {
    primitive: Box<Intersectable + Send + Sync>,
}

impl Inverted {
    pub fn new(primitive: Box<Intersectable + Send + Sync>) -> Inverted {
        Inverted {
            primitive,
        }
    }
}

impl Intersectable for Inverted {
    // TODO: we really need to call get_all_intersects here because the closest inverted intersect
    // probably won't be the closest intersect
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        if let Some(mut intersect) = self.primitive.get_closest_intersect(ray) {
            if ray.direction.dot(intersect.surface_normal) >= 0.0 {
                intersect.surface_normal *= -1.0;
                return Some(intersect);
            }
        }
        None
    }

/*
    fn surface_normal(&self, hit_point: DVec3) -> DVec3 {
        self.primitive.surface_normal(hit_point) * -1.0
    }
    */

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let mut intersects = self.primitive.get_all_intersects(ray);
        for intersect in intersects.iter_mut() {
            intersect.surface_normal *= -1.0;
        }
        intersects
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub radius: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Sphere {
        Sphere {radius}
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
            intersects.0 = Some(Intersect::new(ray, t0, hit_point, hit_point.normalize()));
        } 

        if t1 >= Ray::MIN_DISTANCE {
            let hit_point = ray.point_at_distance(t1);
            intersects.1 = Some(Intersect::new(ray, t1, hit_point, hit_point.normalize()));
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

/*
    fn surface_normal(&self, hit_point: DVec3) -> DVec3 {
        hit_point.normalize()
    }
    */

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
            Some(Intersect::new(ray, hit_distance, hit_point, surface_normal))
        }
        else {
            None
        }
    }

/*
    fn surface_normal(&self, _: DVec3) -> DVec3 {
        dvec3!(0.0, 0.0, 1.0)
    }
    */

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let mut ret_intersects: Vec<Intersect> = Vec::new(); 
        if let Some(intersect) = self.get_closest_intersect(ray) {
            ret_intersects.push(intersect);
        }
        ret_intersects
    }
}

#[derive(Clone)]
pub struct Cube {
    pub length: f64,
    base_plane: RectangularPlane,
    matrices: [DMat4; 6],
    inverse_matrices: [DMat4; 6],
}

impl Cube {
    pub fn new(length: f64) -> Cube {
        let mut matrices: [DMat4; 6] = [DMat4::identity(); 6];  
        let base_plane = RectangularPlane::new(length, length);

        matrices[0] = translation(length/2.0, 0.0, 0.0) * rotation(Axis::Y, 90.0);
        matrices[1] = translation(-length/2.0, 0.0, 0.0) * rotation(Axis::Y, -90.0);
        matrices[2] = translation(0.0, 0.0, length/2.0);
        matrices[3] = translation(0.0, 0.0, -length/2.0) * reflection(Axis::Z);
        matrices[4] = translation(0.0, length/2.0, 0.0) * rotation(Axis::X, -90.0);
        matrices[5] = translation(0.0, -length/2.0, 0.0) * rotation(Axis::X, 90.0);

        let mut inverse_matrices: [DMat4; 6] = [DMat4::identity(); 6];
        for i in 0..6 {
            inverse_matrices[i] = matrices[i].inverse();
        }

        Cube{length, base_plane, matrices, inverse_matrices}
    }

    fn two_intersects(&self, ray: Ray) -> (Option<Intersect>, Option<Intersect>) {
        let mut intersects: (Option<Intersect>, Option<Intersect>) = (None, None);
        for i in 0..6 {
            let transformed_ray = ray.transform(self.inverse_matrices[i]);
            if let Some(intersect) = self.base_plane.get_closest_intersect(transformed_ray) {
                let mut hit_point = transform_point(self.matrices[i], intersect.hit_point);
                let hit_distance = (ray.origin - hit_point).length();
                if hit_distance >= Ray::MIN_DISTANCE {
                    let current_intersect = Intersect::new(ray, hit_distance, hit_point, self.surface_normal(hit_point));
                    if let Some(first_intersect) = intersects.0 {
                        // Swap to maintain order by hit distance
                        if hit_distance < first_intersect.distance {
                            intersects.1 = Some(first_intersect);
                            intersects.0 = Some(current_intersect);
                        }
                        else {
                            intersects.1 = Some(current_intersect);
                        }
                        break;
                    }
                    else {
                            intersects.0 = Some(current_intersect);
                    }
                }
            }
        }
        intersects
    }

    fn surface_normal(&self, hit_point: DVec3) -> DVec3 {
        let mut normal = dvec3!(0.0, 0.0, 0.0);

        let max_yz = hit_point.y.abs().max(hit_point.z.abs());
        if hit_point.x >= max_yz { normal.x = 1.0 }
        else if hit_point.x <= -max_yz { normal.x = -1.0}

        let max_xz = hit_point.x.abs().max(hit_point.z.abs());
        if hit_point.y >= max_xz { normal.y = 1.0 }
        else if hit_point.y <= -max_xz { normal.y = -1.0}

        let max_xy = hit_point.x.abs().max(hit_point.y.abs());
        if hit_point.z >= max_xy { normal.z = 1.0 }
        else if hit_point.z <= -max_xy { normal.z = -1.0}

        normal.normalize()
    }
}

impl Intersectable for Cube {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        self.two_intersects(ray).0
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