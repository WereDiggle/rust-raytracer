use std::path::Path;
use euler::{DVec3, dvec3, DVec2, dvec2};
use std::sync::Arc;
use geometry::{Ray, SurfaceCoord, Intersectable, Intersect};
use geometry::matrix::*;
use primitive::plane::Triangle;

#[derive(Clone)]
struct BoundingBox {
    pub max: DVec3,
    pub min: DVec3,
}

enum Bound {
    Inside,
    Hit(Intersect),
    Miss,
}

impl BoundingBox {
    pub fn bound_nothing() -> BoundingBox {
        BoundingBox{max: NEG_INF, min: INF}
    }

    pub fn expand(&mut self, vertices: &Vec<DVec3>) {
        for v in vertices.iter() {
            self.max = max_bound(self.max, *v);
            self.min = min_bound(self.min, *v);
        }
    }

    pub fn from_positions(positions: Vec<f32>) -> BoundingBox {
        let mut max = dvec3!(positions[0], positions[1], positions[2]);
        let mut min = dvec3!(positions[0], positions[1], positions[2]);
        for p in 0..positions.len()/3 {
            let vertex = dvec3!(positions[3], positions[3*p+1], positions[3*p+2]);
            max = max_bound(max, vertex);
            min = min_bound(min, vertex);
        }
        let err = dvec3!(Ray::MIN_DISTANCE, Ray::MIN_DISTANCE, Ray::MIN_DISTANCE);
        let max_err = max - err;
        let min_err = min + err;
        BoundingBox{max, min}
    }

    fn contains(&self, point: DVec3) -> bool {
        point.x < self.max.x && point.x > self.min.x &&
        point.y < self.max.y && point.y > self.min.y &&
        point.z < self.max.z && point.z > self.min.z
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

struct BoundingTree {
    max: DVec3,
    min: DVec3,
    root: BoundingNode
}

impl BoundingTree {
    pub fn from_triangles(faces: &Vec<(usize, usize, usize)>, vertices: &Vec<DVec3>) -> BoundingTree {
        let mut prims: Vec<(usize, DVec3, DVec3)> = Vec::with_capacity(faces.len());
        let mut total_max = NEG_INF;
        let mut total_min = INF; 
        for (i, face) in faces.iter().enumerate() {
            let a = vertices[face.0];
            let b = vertices[face.1];
            let c = vertices[face.2];
            let max = max_bound(a, max_bound(b, c));
            let min = min_bound(a, min_bound(b, c));
            total_max = max_bound(max, total_max);
            total_min = min_bound(min, total_min);
            prims.push((i, min, max));
        }
        let root = BoundingNode::split_node(Axis::X, BoundingNode::MAX_DEPTH, prims);
        BoundingTree {
            max: total_max,
            min: total_min,
            root,
        }
    }

    pub fn check_intersect(&self, mesh: &Mesh, ray: Ray) -> Option<Intersect> {
        self.root.check_intersect(mesh, ray, self.min, self.max)
    }
}

#[derive(Clone)]
enum BoundingNode {
    Interior{
        axis: Axis,                 // Axis of split
        split: f64,                 // position of split plane
        child: [Box<BoundingNode>; 2],   // children node
    },
    Leaf{
        prims: Vec<usize>,          // indicies to primitives
    },
}

impl BoundingNode {
    pub const MAX_PRIMS: usize = 16;
    pub const MAX_DEPTH: u8 = 30;

    pub fn check_intersect(&self, mesh: &Mesh, ray: Ray, min: DVec3, max: DVec3) -> Option<Intersect> {
        match self {
            BoundingNode::Interior{axis, split, child} => {

                // First check if the ray intersects with this node at all
                let bounding_box = BoundingBox{min, max};
                if let Bound::Hit(intersect) = bounding_box.check_intersect(ray) {
                    // Figure out the new mins and maxs
                    let mut split_min = min;
                    let mut split_max = max;
                    match axis {
                        Axis::X => {
                            split_min.x = *split;
                            split_max.x = *split;
                        },
                        Axis::Y => {
                            split_min.y = *split;
                            split_max.y = *split;
                        },
                        Axis::Z => {
                            split_min.z = *split;
                            split_max.z = *split;
                        },
                    }

                    // Then figure out which child it goes through first
                    let first = axis.value(intersect.hit_point) <= *split;
                    let same_side = |x| {
                        if first {
                            x <= *split
                        }    
                        else {
                            x > *split
                        }
                    };

                    let (first, first_min, first_max, second, second_min, second_max) = 
                    if first {
                        (0, min, split_max, 1, split_min, max)
                    }
                    else {
                        (1, split_min, max, 0, min, split_max)
                    };

                    let mut min_distance = INF.x;
                    // If there's an intersection from that child that's doesn't cross the split
                    if let Some(intersect) = child[first].check_intersect(mesh, ray, first_min, first_max) {
                        if same_side(axis.value(intersect.hit_point)) {
                            // return that intersect
                            return Some(intersect);
                        }
                        else {
                            min_distance = intersect.distance;
                        }
                    }

                    // At this point, we haven't returned so we need to check the other child
                    if let Some(intersect) = child[second].check_intersect(mesh, ray, second_min, second_max) {
                        if intersect.distance < min_distance {
                            return Some(intersect);
                        }
                    }
                }
                None
            },
            BoundingNode::Leaf{prims} => {
                // Check triangles of all prims 
                let min_distance = INF.x;
                let mut ret_int: Option<Intersect> = None;
                for prim in prims.iter() {
                    if let Some(intersect) = mesh.check_triangle(*prim, ray) {
                        if intersect.distance < min_distance {
                            ret_int = Some(intersect);
                        }
                    }
                }
                ret_int
            },
        }
    }

    fn split_node(axis: Axis, 
                  depth: u8, 
                  mut prims: Vec<(usize, DVec3, DVec3)>)
                  -> BoundingNode {

        if depth == 0 || prims.len() <= BoundingNode::MAX_PRIMS {
            BoundingNode::Leaf{
                prims: prims.iter().map(|x| x.0).collect()
            }
        }
        else {
            // Sort prims by axis
            prims.sort_by(|a, b| {
                axis.value(a.1).partial_cmp(&axis.value(b.1)).unwrap()
            });

            // Split prims into two
            let split = prims.len() / 2;
            let mut child2_prims = prims.split_off(split);

            // Calculate position of split
            let split = axis.value(child2_prims[0].1);

            // Make sure child2_prims is inclusive of upper bounds
            let mut prim_overlap = 0;
            for prim in prims.iter() {
                // Comparing upper bounds to split
                if axis.value(prim.2) >= split {
                    child2_prims.push(prim.clone());
                    prim_overlap+=1;
                }
            }

            // TODO: remove
            if prim_overlap > 0 {
                println!("primitive overlap at depth {}: {}", depth, child2_prims.len() - prims.len());
            }

            // Create Bounding node with recursive call to create children
            let axis = match axis {
                Axis::X => Axis::Y,
                Axis::Y => Axis::Z,
                Axis::Z => Axis::X,
            };

            BoundingNode::Interior {
                axis,
                split,
                child: [
                    Box::new(BoundingNode::split_node(axis, depth-1, prims)),
                    Box::new(BoundingNode::split_node(axis, depth-1, child2_prims)),
                ],
            }
        }
    }
}

#[derive(Clone)]
struct Face {
    v: [usize; 3],
    n: [usize; 3],
}

#[derive(Clone)]
pub struct Mesh {
    pub positions: Arc<Vec<DVec3>>,
    pub vertex_normals: Arc<Vec<DVec3>>,
    pub tex_coords: Arc<Vec<DVec2>>,
    pub faces: Arc<Vec<(usize, usize, usize)>>,
    face_normals: Arc<Vec<DVec3>>,
    face_area: Arc<Vec<f64>>,
    bounds: Arc<BoundingTree>,
}

fn f32_to_dvec3(positions: &Vec<f32>) -> Vec<DVec3> {
    assert!(positions.len() % 3 == 0);

    let mut ret_vec: Vec<DVec3> = Vec::with_capacity(positions.len() / 3);
    for i in 0..positions.len() / 3 {
        ret_vec.push(dvec3!(positions[3*i], positions[3*i+1], positions[3*i+2]));
    }
    ret_vec
}

fn f32_to_dvec2(positions: &Vec<f32>) -> Vec<DVec2> {
    assert!(positions.len() % 2 == 0);

    let mut ret_vec: Vec<DVec2> = Vec::with_capacity(positions.len() / 2);
    for i in 0..positions.len() / 2 {
        ret_vec.push(dvec2!(positions[2*i], positions[2*i+1]));
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
        let mut vertex_normals: Vec<DVec3> = vec!();
        let mut tex_coords: Vec<DVec2> = vec!();
        for m in models.iter() {
            let mesh = &m.mesh;
            faces.append(&mut indices_to_faces(positions.len(), &mesh.indices));
            positions.append(&mut f32_to_dvec3(&mesh.positions));
            vertex_normals.append(&mut f32_to_dvec3(&mesh.normals));
            tex_coords.append(&mut f32_to_dvec2(&mesh.texcoords));
        }
        let mut face_normals: Vec<DVec3> = Vec::with_capacity(faces.len());
        let mut face_area: Vec<f64> = Vec::with_capacity(faces.len());
        for (f1, f2, f3) in faces.iter() {
            let a = (positions[*f2]-positions[*f1]).cross(positions[*f3]-positions[*f2]);
            face_normals.push(a.normalize());

            // only used for division, so divide first since multiplication is faster
            face_area.push(1.0/a.length());
        }
        let bounds = Arc::new(BoundingTree::from_triangles(&faces, &positions));
        let positions = Arc::new(positions);
        let vertex_normals = Arc::new(vertex_normals);
        let tex_coords = Arc::new(tex_coords);
        let faces = Arc::new(faces);
        let face_normals = Arc::new(face_normals);
        let face_area = Arc::new(face_area);
        
        Box::new(Mesh{positions, vertex_normals, tex_coords, faces, face_normals, face_area, bounds})
    }

    pub fn check_triangle(&self, face: usize, ray: Ray) -> Option<Intersect> {
        let (i1, i2, i3) = self.faces[face];
        let v1 = self.positions[i1];
        let v2 = self.positions[i2];
        let v3 = self.positions[i3];
        let e1 = v2 - v1;
        let e2 = v3 - v2;

        let normal = self.face_normals[face];
        let origin_dot = normal.dot(ray.origin - v1);
        let direction_dot = normal.dot(ray.direction);
        let hit_distance = -origin_dot / direction_dot;
        if hit_distance > Ray::MIN_DISTANCE {
            let hit_point = ray.point_at_distance(hit_distance);

            let hit_v1 = hit_point - v1;
            let hit_v2 = hit_point - v2;
            let hit_v3 = hit_point - v3;
            let e3 = v1 - v3;

            let mut v = e1.cross(hit_v1);
            if v.dot(normal) < 0.0 {return None;}

            let mut w = e2.cross(hit_v2);
            if w.dot(normal) < 0.0 {return None;}

            let mut u = e3.cross(hit_v3);
            if u.dot(normal) < 0.0 {return None;}

            let u = u.length() * self.face_area[face];
            let v = v.length() * self.face_area[face];
            let w = 1.0 - u - v;

            let normal = if self.vertex_normals.len() > i1 {
                w*self.vertex_normals[i1] + u*self.vertex_normals[i2] + v*self.vertex_normals[i3]
            } else {
                normal
            };
            
            let surface_coord = if self.tex_coords.len() > i1 {
                let tmp = w*self.tex_coords[i1] + u*self.tex_coords[i2] + v*self.tex_coords[i3];
                SurfaceCoord::new(tmp.x, tmp.y)
            } else {
                SurfaceCoord::new(0.0, 0.0)
            };

            return Some(Intersect::new(ray, hit_distance, hit_point, normal, UP, surface_coord));
        }

        None
    }
}

impl Intersectable for Mesh {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        // TODO: bounds and mesh are weirdly coupled, it makes me uncomfortable
        self.bounds.check_intersect(self, ray)
    }
}