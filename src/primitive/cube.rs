use super::*;

#[derive(Clone)]
pub struct Cube {
    pub length: f64,
    base_plane: Box<Rectangle>,
    matrices: [DMat4; 6],
    inverse_matrices: [DMat4; 6],
}

impl Cube {
    const texture_offsets: [(f64, f64); 6] = [(2.0, 1.0), (0.0, 1.0), (1.0, 1.0), (3.0, 1.0), (1.0, 2.0), (1.0, 0.0)];

    pub fn new(length: f64) -> Box<Cube> {
        let mut matrices: [DMat4; 6] = [DMat4::identity(); 6];  
        let base_plane = Rectangle::new(length, length);

        matrices[0] = rotation(Axis::Y, 90.0) * translation(0.0, 0.0, length/2.0);      // right
        matrices[1] = rotation(Axis::Y, -90.0) * translation(0.0, 0.0, length/2.0);     // left
        matrices[2] = translation(0.0, 0.0, length/2.0);                                // front
        matrices[3] = rotation(Axis::Y, 180.0) * translation(0.0, 0.0, length/2.0);     // back
        matrices[4] = rotation(Axis::X, -90.0) * translation(0.0, 0.0, length/2.0);     // top
        matrices[5] = rotation(Axis::X, 90.0) * translation(0.0, 0.0, length/2.0);      // bottom

        let mut inverse_matrices: [DMat4; 6] = [DMat4::identity(); 6];
        for i in 0..6 {
            inverse_matrices[i] = matrices[i].inverse();
        }

        Box::new(Cube{length, base_plane, matrices, inverse_matrices})
    }

    fn transform_surface_coord(surface_coord: SurfaceCoord, face: usize) -> SurfaceCoord {
        let (mut u, mut v) = surface_coord.get_coord();
        u = (u + Cube::texture_offsets[face].0)/4.0;
        v = (v + Cube::texture_offsets[face].1)/3.0;
        SurfaceCoord::new(u, v)
    }

    fn two_intersects(&self, ray: Ray) -> (Option<Intersect>, Option<Intersect>) {
        let mut intersects: (Option<Intersect>, Option<Intersect>) = (None, None);
        for i in 0..6 {

            let transformed_ray = ray.transform(self.inverse_matrices[i]);
            if let Some(intersect) = self.base_plane.get_closest_intersect(transformed_ray) {

                let mut hit_point = transform_point(self.matrices[i], intersect.hit_point);
                let hit_distance = (ray.origin - hit_point).length();

                if hit_distance >= Ray::MIN_DISTANCE {
                    let surface_coord = Cube::transform_surface_coord(intersect.surface_coord, i);
                    let current_intersect = Intersect::new(ray, hit_distance, hit_point, self.surface_normal(hit_point), surface_coord);
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