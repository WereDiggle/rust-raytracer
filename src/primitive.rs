use euler::{dvec3, DVec3, DMat4};
use geometry::{Intersectable, Ray, matrix::*};

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
    fn check_intersect(&self, ray: Ray) -> Option<f64> {
        if let Some(distance) = self.primitive.check_intersect(ray) {
            let surface_normal = self.primitive.surface_normal(ray.point_at_distance(distance));
            if ray.direction.dot(surface_normal) < 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, hit_point: DVec3) -> DVec3 {
        self.primitive.surface_normal(hit_point)
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
    fn check_intersect(&self, ray: Ray) -> Option<f64> {
        if let Some(distance) = self.primitive.check_intersect(ray) {
            let surface_normal = self.primitive.surface_normal(ray.point_at_distance(distance));
            if ray.direction.dot(surface_normal) >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, hit_point: DVec3) -> DVec3 {
        self.primitive.surface_normal(hit_point) * -1.0
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
}

impl Intersectable for Sphere {
    fn check_intersect(&self, ray: Ray) -> Option<f64> {
        let l = -1.0 * ray.origin;
        let adj = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < Ray::MIN_DISTANCE && t1 < Ray::MIN_DISTANCE {
            None
        } else if t0 < Ray::MIN_DISTANCE {
            Some(t1)
        } else if t1 < Ray::MIN_DISTANCE {
            Some(t0)
        }
        else {
            Some(if t0<t1 {t0} else {t1})
        }
    }

    fn surface_normal(&self, hit_point: DVec3) -> DVec3 {
        hit_point.normalize()
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
    fn check_intersect(&self, ray: Ray) -> Option<f64> {
        // Get point on the plane
        let normal = dvec3!(0.0, 0.0, 1.0);
        let hit_distance = ray.origin.dot(normal) / ray.direction.dot(normal) * -1.0;
        let point_hit = ray.point_at_distance(hit_distance);

        // Check point against bounds
        let horizontal_bound = self.width/2.0;
        let vertical_bound = self.height/2.0;
        if point_hit.x < -horizontal_bound || point_hit.x > horizontal_bound ||
            point_hit.y < -vertical_bound || point_hit.y > vertical_bound {
            return None;
        }

        if hit_distance >= Ray::MIN_DISTANCE {
            Some(hit_distance)
        }
        else {
            None
        }
    }

    fn surface_normal(&self, _: DVec3) -> DVec3 {
        dvec3!(0.0, 0.0, 1.0)
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
}

impl Intersectable for Cube {
    fn check_intersect(&self, ray: Ray) -> Option<f64> {
        let mut final_distance: Option<f64> = None;
        for i in 0..6 {
            let transformed_ray = ray.transform(self.inverse_matrices[i]);
            if let Some(distance) = self.base_plane.check_intersect(transformed_ray) {
                let mut hit_point = transformed_ray.point_at_distance(distance);
                hit_point = transform_point(self.matrices[i], hit_point);
                let hit_distance = (ray.origin - hit_point).length();
                if let Some(final_dist) = final_distance {
                    if hit_distance >= Ray::MIN_DISTANCE && hit_distance < final_dist {
                        final_distance = Some(hit_distance);
                    }
                }
                else {
                    if hit_distance >= Ray::MIN_DISTANCE {
                        final_distance = Some(hit_distance);
                    }
                }
            }
        }
        final_distance
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