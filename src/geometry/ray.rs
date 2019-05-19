use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
    depth: u32,
    contribution: Color,
}

impl Ray {
    pub const MIN_DISTANCE: f64 = 0.001;
    pub const MIN_CONTRIBUTION: f64 = 0.003;
    pub fn new(origin: DVec3, direction: DVec3, depth: u32) -> Ray {
        Ray { origin, direction, depth, contribution: Color::from_f64(1.0) }
    }

    pub fn from_destination(origin: DVec3, destination: DVec3, depth: u32) -> Ray {
        Ray { origin, direction: (destination - origin).normalize(), depth, contribution: Color::from_f64(1.0) }
    }

    pub fn contributes(&self, percentage: Color) -> Ray {
        let mut new_ray = self.clone();
        new_ray.contribution *= percentage;
        new_ray
    }

    pub fn get_contribution(&self) -> f64 {
        self.contribution.red +
        self.contribution.green +
        self.contribution.blue
    }

    pub fn get_depth(&self) -> u32 {
        self.depth
    }

    pub fn reflect_off(&self, hit_point: DVec3, surface_normal: DVec3) -> Ray {
        assert!(surface_normal.length() - 1.0 < 0.000001);
        // math
        let dot = self.direction.dot(surface_normal);
        //assert!(dot <= 0.0);

        let reflection_direction = (self.direction - 2.0 * dot * surface_normal).normalize();

        assert!(self.depth > 0);
        Ray {
            origin: hit_point,
            direction: reflection_direction,
            depth: self.depth - 1,
            contribution: self.contribution,
        }
    }

    // TODO: should this be in Ray or TranslucentShader?
    pub fn transmit_through(&self, hit_point: DVec3, mut surface_normal: DVec3, mut refractive_index: f64) -> Ray {
        assert!(surface_normal.length() - 1.0 < 0.000001);
        assert!(self.depth > 0);

        let mut incident_cos = self.direction.dot(surface_normal);

        if incident_cos < 0.0 {
            incident_cos = -incident_cos;
            refractive_index = 1.0/refractive_index;
        }
        else {
            surface_normal = surface_normal * -1.0;
        }

        let refraction_factor = 1.0 - refractive_index*refractive_index * (1.0 - incident_cos*incident_cos);

        if refraction_factor < 0.0 {
            self.reflect_off(hit_point, surface_normal)
        }
        else {
            let refraction_direction = refractive_index * self.direction + (refractive_index*incident_cos-refraction_factor.sqrt()) * surface_normal;
            Ray {
                origin: hit_point,
                direction: refraction_direction,
                depth: self.depth - 1,
                contribution: self.contribution,
            }
        }
    }

    pub fn point_at_distance(&self, distance: f64) -> DVec3 {
        self.origin + (distance * self.direction)
    }

    pub fn transform(&self, matrix: DMat4) -> Ray {
        Ray { 
            origin: (matrix * dvec4!(self.origin, 1.0)).xyz(),
            direction: (matrix * dvec4!(self.direction, 0.0)).xyz().normalize(),
            depth: self.depth,
            contribution: self.contribution,
        }
    }
}
