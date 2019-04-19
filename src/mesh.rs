use std::path::Path;
use euler::{DVec3, dvec3};
use std::sync::Arc;
use geometry::{Ray, SurfaceCoord, Intersectable, Intersect};
use geometry::matrix::*;
use primitive::plane::Triangle;

#[derive(Clone)]
struct BoundingBox {
    pub max: DVec3,
    pub min: DVec3,
    max_err: DVec3,
    min_err: DVec3,
}

enum Bound {
    Inside,
    Hit(Intersect),
    Miss,
}

impl BoundingBox {
    pub fn bound_nothing() -> BoundingBox {
        BoundingBox{max: NEG_INF, min: INF, max_err: NEG_INF, min_err: INF}
    }

    pub fn expand(&mut self, vertices: &Vec<DVec3>) {
        for v in vertices.iter() {
            self.max.x = self.max.x.max(v.x);
            self.max.y = self.max.y.max(v.y);
            self.max.z = self.max.z.max(v.z);

            self.min.x = self.min.x.min(v.x);
            self.min.y = self.min.y.min(v.y);
            self.min.z = self.min.z.min(v.z);
        }
        let err = dvec3!(Ray::MIN_DISTANCE, Ray::MIN_DISTANCE, Ray::MIN_DISTANCE);
        self.max_err = self.max - err;
        self.min_err = self.min + err;
    }

    pub fn from_positions(positions: Vec<f32>) -> BoundingBox {
        let mut max = dvec3!(positions[0], positions[1], positions[2]);
        let mut min = dvec3!(positions[0], positions[1], positions[2]);
        for p in 0..positions.len()/3 {
            max.x = max.x.max(positions[3*p] as f64);
            max.y = max.y.max(positions[3*p+1] as f64);
            max.z = max.z.max(positions[3*p+2] as f64);

            min.x = min.x.min(positions[3*p] as f64);
            min.y = min.y.min(positions[3*p+1] as f64);
            min.z = min.z.min(positions[3*p+2] as f64);
        }
        let err = dvec3!(Ray::MIN_DISTANCE, Ray::MIN_DISTANCE, Ray::MIN_DISTANCE);
        let max_err = max - err;
        let min_err = min + err;
        BoundingBox{max, min, max_err, min_err}
    }

    fn contains(&self, point: DVec3) -> bool {
        point.x < self.max_err.x && point.x > self.min_err.x &&
        point.y < self.max_err.y && point.y > self.min_err.y &&
        point.z < self.max_err.z && point.z > self.min_err.z
    }

    fn contains_xy(&self, point: DVec3) -> bool {
        point.x <= self.max.x && point.x >= self.min.x &&
        point.y <= self.max.y && point.y >= self.min.y
    }

    fn contains_xz(&self, point: DVec3) -> bool {
        point.x <= self.max.x && point.x >= self.min.x &&
        point.z <= self.max.z && point.z >= self.min.z
    }

    fn contains_yz(&self, point: DVec3) -> bool {
        point.y <= self.max.y && point.y >= self.min.y &&
        point.z <= self.max.z && point.z >= self.min.z
    }

    fn check_axis<F>(&self, ray: Ray, axis: DVec3, tangent: DVec3, check: F) -> Option<Bound>
        where F: Fn(&BoundingBox, DVec3) -> bool
    {
        let axis_dot = ray.direction.dot(axis);
        if axis_dot < 0.0 {
            let origin_dot = axis.dot(ray.origin - self.max);
            if origin_dot >= 0.0 {
                let distance = -origin_dot / axis_dot;
                let hit_point = ray.point_at_distance(distance);
                if check(self, hit_point) {
                    return Some(Bound::Hit(Intersect::new(
                        ray, distance, hit_point, axis, tangent, SurfaceCoord::new(0.0, 0.0)
                    )));
                }
            }
        }
        else if axis_dot > 0.0 {
            let origin_dot = -axis.dot(ray.origin - self.min);
            if origin_dot >= 0.0 {
                let distance = origin_dot / axis_dot;
                let hit_point = ray.point_at_distance(distance);
                if check(self, hit_point) {
                    return Some(Bound::Hit(Intersect::new(
                        ray, distance, hit_point, -1.0*axis, -1.0*tangent, SurfaceCoord::new(0.0, 0.0)
                    )));
                }
            }
        }
        None
    }

    pub fn check_intersect(&self, ray: Ray) -> Bound {
        if self.contains(ray.origin) {
            return Bound::Inside;
        }

        // Test back/front
        if let Some(bound) = self.check_axis(ray, FRONT, UP, BoundingBox::contains_xy) {
            return bound;
        }
        if let Some(bound) = self.check_axis(ray, RIGHT, UP, BoundingBox::contains_yz) {
            return bound;
        }
        if let Some(bound) = self.check_axis(ray, UP, FRONT, BoundingBox::contains_xz) {
            return bound;
        }
        Bound::Miss
    }
}

#[derive(Clone)]
pub struct Mesh {
    // TODO: store positions separately
    pub positions: Arc<Vec<DVec3>>,
    pub faces: Arc<Vec<(usize, usize, usize)>>,
    bounds: BoundingBox,
}

fn position_to_dvec3(positions: &Vec<f32>) -> Vec<DVec3> {
    assert!(positions.len() % 3 == 0);

    let mut ret_vec: Vec<DVec3> = Vec::with_capacity(positions.len() / 3);
    for i in 0..positions.len() / 3 {
        ret_vec.push(dvec3!(positions[3*i], positions[3*i+1], positions[3*i+2]));
    }
    ret_vec
}

fn indices_to_faces(offset: usize, indices: &Vec<u32>) -> Vec<(usize, usize, usize)> {
    assert!(indices.len() % 3 == 0);

    let mut ret_vec: Vec<(usize, usize, usize)> = Vec::with_capacity(indices.len() / 3);
    for i in 0..indices.len() / 3 {
        ret_vec.push((offset+indices[3*i] as usize, offset+indices[3*i+1] as usize, offset+indices[3*i+2] as usize));
    }
    ret_vec
}

impl Mesh {
    pub fn from_path(path: &Path) -> Box<Mesh> {
        let (models, materials) = tobj::load_obj(path).unwrap();
        let mut faces: Vec<(usize, usize, usize)> = vec!();
        let mut positions: Vec<DVec3> = vec!();
        let mut bounds = BoundingBox::bound_nothing();
        for m in models.iter() {
            let mesh = &m.mesh;
            faces.append(&mut indices_to_faces(positions.len(), &mesh.indices));
            positions.append(&mut position_to_dvec3(&mesh.positions));
        }
        bounds.expand(&positions);
        let faces = Arc::new(faces);
        let positions = Arc::new(positions);
        Box::new(Mesh{positions, faces, bounds})
    }

    fn check_triangle(&self, face: usize, ray: Ray) -> Option<Intersect> {
        // Only comparing against one of the vertices
        // So if the ray origin is waaay close, might not intersect when it should
        let (v1, v2, v3) = self.faces[face];
        let v1 = self.positions[v1];
        let v2 = self.positions[v2];
        let v3 = self.positions[v3];
        let e1 = v2 - v1;
        let e2 = v3 - v2;

        // TODO: calc surface normal for now, we'll be storing it in the future when
        // we add interpolated normals
        let normal = e1.cross(e2).normalize();
        let origin_dot = normal.dot(ray.origin - v1);
        let direction_dot = normal.dot(ray.direction);
        let hit_distance = -origin_dot / direction_dot;
        if hit_distance > Ray::MIN_DISTANCE {
            let hit_point = ray.point_at_distance(hit_distance);

            let vector_v1 = hit_point - v1;
            let vector_v2 = hit_point - v2;
            let vector_v3 = hit_point - v3;
            let e3 = v1 - v3;

            if e1.cross(vector_v1).dot(normal) >= 0.0 &&
               e2.cross(vector_v2).dot(normal) >= 0.0 &&
               e3.cross(vector_v3).dot(normal) >= 0.0 {

                   // TODO: surface coords for a mesh
                   let surface_coord = SurfaceCoord::new(0.0, 0.0);
                   return Some(Intersect::new(ray, hit_distance, hit_point, normal, UP, surface_coord));
            }
        }

        None
    }
}

impl Intersectable for Mesh {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        // TODO: we can cut out all away facing triangles if the ray starts from outside the bounding box

        match self.bounds.check_intersect(ray) {
            Bound::Inside | Bound::Hit(_) => {
                let mut ret_intersect: Intersect = Intersect::at_infinity(ray);
                for i in 0..self.faces.len() {
                    if let Some(intersect) = self.check_triangle(i, ray) {
                        if intersect.distance < ret_intersect.distance {
                            ret_intersect = intersect;
                        }
                    }
                }
                if ret_intersect.distance == std::f64::INFINITY {
                    None
                }
                else {
                    Some(ret_intersect)
                }
            },
            Bound::Miss => None,
        }
    }
}