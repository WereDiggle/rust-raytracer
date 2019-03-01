use euler::{DVec2, dvec2};

#[derive(Clone, Copy)]
pub struct SurfaceCoord {
    coord: DVec2,
}

impl SurfaceCoord {
    pub fn new(u: f64, v: f64) -> SurfaceCoord {
        SurfaceCoord {coord: dvec2!(u, v)}
    }

    pub fn get_u(&self) -> f64 {self.coord.x}
    pub fn get_v(&self) -> f64 {self.coord.y}
    pub fn get_coord(&self) -> (f64, f64) {(self.coord.x, self.coord.y)}
}
