use super::*;

#[derive(Clone, Copy)]
pub struct Intersect {
    pub ray: Ray,
    pub distance: f64, 
    pub hit_point: DVec3,
    pub surface_normal: DVec3,
    pub surface_coord: SurfaceCoord,
}

impl Intersect {
    pub fn new(ray: Ray, distance: f64, hit_point: DVec3, surface_normal: DVec3, surface_coord: SurfaceCoord) -> Intersect {
        Intersect{ ray, distance, hit_point, surface_normal, surface_coord}
    }

    pub fn transform(&self, matrix: DMat4) -> Intersect {
        let ray = self.ray.transform(matrix);
        let hit_point = matrix::transform_point(matrix, self.hit_point);
        let distance = (hit_point - ray.origin).length();
        let surface_normal = matrix::transform_normal(matrix, self.surface_normal);
        Intersect {
            ray,
            distance,
            hit_point,
            surface_normal,
            surface_coord: self.surface_coord,
        }
    }

    pub fn contributes(&self, percentage: Color) -> Intersect {
        Intersect {
            ray: self.ray.contributes(percentage),
            distance: self.distance,
            hit_point: self.hit_point,
            surface_normal: self.surface_normal,
            surface_coord: self.surface_coord,
        }
    }

    pub fn invert_normal(&self) -> Intersect {
        Intersect {
            ray: self.ray,
            distance: self.distance,
            hit_point: self.hit_point,
            surface_normal: self.surface_normal * -1.0,
            surface_coord: self.surface_coord,
        }
    }
}

#[derive(Clone, Copy)]
pub struct NodeIntersect<'a> {
    pub hit_id: ProcessUniqueId,
    pub shader: &'a (Shadable + Send + Sync + 'a),
    pub intersect: Intersect,
}

impl<'a> NodeIntersect<'a> {
    pub fn new(hit_id: ProcessUniqueId, shader: &'a (Shadable + Send + Sync + 'a), intersect: Intersect) -> NodeIntersect<'a> {
        NodeIntersect {hit_id, shader, intersect}
    }

    pub fn transform(&self, matrix: DMat4) -> NodeIntersect<'a> {
        NodeIntersect {
            hit_id: self.hit_id,
            shader: self.shader,
            intersect: self.intersect.transform(matrix),
        }
    }

    pub fn get_distance(&self) -> f64 {
        self.intersect.distance
    }

    pub fn get_hit_point(&self) -> DVec3 {
        self.intersect.hit_point
    }

    pub fn get_surface_normal(&self) -> DVec3 {
        self.intersect.surface_normal
    }

    pub fn get_ray(&self) -> Ray {
        self.intersect.ray
    }
}