use super::*;

#[derive(Clone)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Box<Rectangle> {
        Box::new(Rectangle{width, height})
    }
}

impl Intersectable for Rectangle {
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
}

#[derive(Clone)]
pub struct Triangle {
    pub vertices: [DVec3; 3],
    edges: [DVec3; 3],
    surface_coord_height: f64,
    surface_coord_width: f64,
}

impl Triangle {
    pub fn from_vertices(v1: DVec3, v2: DVec3, v3: DVec3) -> Box<Triangle> {
        let edges = [v2-v1, v3-v2, v1-v3];
        let a_side = -1.0*edges[2];
        let b_side = edges[0];
        let surface_coord_height = a_side - (a_side.dot(b_side)/(b_side).dot(b_side))*b_side;
        let surface_coord_width = a_side - surface_coord_height;
        let surface_coord_height = surface_coord_height.length();
        let surface_coord_width = surface_coord_width.length().max(b_side.length());

        Box::new(Triangle{vertices: [v1, v2, v3], edges, surface_coord_height, surface_coord_width})
    }

    // basically using cosine law with lengths of triangle to find coords
    fn get_surface_coord(&self, side_a: f64, side_b: f64) -> SurfaceCoord {
        let side_c = self.edges[0].length();
        let cos_theta = (side_c*side_c + side_a*side_a - side_b*side_b) / (2.0*side_c*side_a);

        let x = side_a * cos_theta;
        let y = (side_a*side_a - x*x).sqrt();
        SurfaceCoord::new(x/self.surface_coord_width, y/self.surface_coord_height)
    }
}

// TODO: generalize this for any polygon
impl Intersectable for Triangle {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        let edge_a = self.vertices[1] - self.vertices[0];
        let edge_b = self.vertices[2] - self.vertices[1];

        let normal = edge_a.cross(edge_b).normalize();

        // Only comparing against one of the vertices
        // So if the ray origin is waaay close, might not intersect when it should
        let origin_dot = normal.dot(ray.origin - self.vertices[0]);
        let direction_dot = normal.dot(ray.direction);
        let hit_distance = -origin_dot / direction_dot;
        if hit_distance > Ray::MIN_DISTANCE {
            let hit_point = ray.point_at_distance(hit_distance);

            let edge_c = self.vertices[0] - self.vertices[2];

            let vector_v1 = hit_point - self.vertices[0];
            let vector_v2 = hit_point - self.vertices[1];
            let vector_v3 = hit_point - self.vertices[2];

            if edge_a.cross(vector_v1).dot(normal) >= 0.0 &&
               edge_b.cross(vector_v2).dot(normal) >= 0.0 &&
               edge_c.cross(vector_v3).dot(normal) >= 0.0 {

                   let surface_coord = self.get_surface_coord(vector_v1.length(), vector_v2.length());
                   return Some(Intersect::new(ray, hit_distance, hit_point, normal, surface_coord));
            }
        }

        None
    }
}

#[derive(Clone)]
pub struct Polygon {
    pub vertices: Vec<DVec3>,
    edges: Vec<DVec3>,
    normal: DVec3,
}

// TODO: put this somewhere else?
fn project_point_onto_plane(point: DVec3, normal: DVec3, origin: DVec3) -> DVec3 {
    point - (point - origin).dot(normal) * normal
}

impl Polygon {
    pub fn from_vertices(mut vertices: Vec<DVec3>) -> Box<Polygon> {
        assert!(vertices.len() >= 3);
        let mut edges: Vec<DVec3> = Vec::with_capacity(vertices.len());

        // Plane is defined by the first two and last vertix
        for i in 0..2 {
            edges.push(vertices[i] - vertices[(i-1)%vertices.len()])
        }

        // TODO: check cross order
        let normal = edges[0].cross(edges[1]).normalize();

        // Project all other vertices onto the plane
        for i in 2..vertices.len()-1 {
            vertices[i] = project_point_onto_plane(vertices[i], normal, vertices[0]);
        }

        for i in 2..vertices.len() {
            edges.push(vertices[i] - vertices[(i-1)%vertices.len()])
        }
        
        Box::new(Polygon{vertices, edges, normal})
    }
}

// TODO: generalize this for any polygon
impl Intersectable for Polygon {
    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        None
    }
}