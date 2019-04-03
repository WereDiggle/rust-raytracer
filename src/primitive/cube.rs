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

                let mut current_intersect = intersect.transform(self.matrices[i]);
                if current_intersect.distance >= Ray::MIN_DISTANCE {
                    current_intersect.surface_coord = Cube::transform_surface_coord(intersect.surface_coord, i);
                    if let Some(first_intersect) = intersects.0 {

                        // Swap to maintain order by hit distance
                        if current_intersect.distance < first_intersect.distance {
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

#[derive(Clone)]
pub struct Tetrahedron {
    // length of base of triangle.
    pub size: f64,
    base_plane: Box<Triangle>,
    matrices: [DMat4; 4],
    inverse_matrices: [DMat4; 4],
}

impl Tetrahedron {
    //TODO: will probably want to implement some sort of texture transformation system
    //const texture_offsets: [(f64, f64); 6] = [(2.0, 1.0), (0.0, 1.0), (1.0, 1.0), (3.0, 1.0), (1.0, 2.0), (1.0, 0.0)];

    pub fn new(size: f64) -> Box<Tetrahedron> {
        use std::f64::consts::FRAC_PI_3;
        let mut matrices: [DMat4; 4] = [DMat4::identity(); 4];  
        let tri_height = size * FRAC_PI_3.sin();
        let base_plane = Triangle::from_vertices(dvec3!(-size/2.0, 0.0, 0.0), 
                                                 dvec3!(size/2.0, 0.0, 0.0),
                                                 dvec3!(0.0, 0.0, -tri_height));

        matrices[0] = reflection(Axis::Y) * translation(0.0, 0.0, tri_height/3.0);                                          // base
        matrices[1] = translation(0.0, 0.0, tri_height/3.0) * rotation(Axis::X, (1.0/3.0 as f64).acos().to_degrees());      // front
        matrices[2] = rotation(Axis::Y, -120.0) * matrices[1];                                                  // left
        matrices[3] = rotation(Axis::Y, 120.0) * matrices[1];                                                  // right

        let mut inverse_matrices: [DMat4; 4] = [DMat4::identity(); 4];
        for i in 0..4 {
            inverse_matrices[i] = matrices[i].inverse();
        }

        Box::new(Tetrahedron{size, base_plane, matrices, inverse_matrices})
    }

    fn transform_surface_coord(surface_coord: SurfaceCoord, face: usize) -> SurfaceCoord {
        let (mut u, mut v) = surface_coord.get_coord();
        u = (u + (face as f64)/2.0)/2.5;
        v = 1.0 - v;
        SurfaceCoord::new(u, v)
    }

    fn two_intersects(&self, ray: Ray) -> (Option<Intersect>, Option<Intersect>) {
        let mut intersects: (Option<Intersect>, Option<Intersect>) = (None, None);
        // TODO: common code with cube, move to common place
        // We're probably going to be using it a lot if we're adding more
        // Polyhedron into this thing
        for i in 0..4 {

            let transformed_ray = ray.transform(self.inverse_matrices[i]);
            if let Some(intersect) = self.base_plane.get_closest_intersect(transformed_ray) {

                let mut current_intersect = intersect.transform(self.matrices[i]);
                if current_intersect.distance >= Ray::MIN_DISTANCE {
                    current_intersect.surface_coord = Tetrahedron::transform_surface_coord(intersect.surface_coord, i);
                    if let Some(first_intersect) = intersects.0 {

                        // Swap to maintain order by hit distance
                        if current_intersect.distance < first_intersect.distance {
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
}

impl Intersectable for Tetrahedron {
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