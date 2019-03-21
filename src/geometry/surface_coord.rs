use euler::{DVec2, dvec2};

#[derive(Clone, Copy)]
pub struct SurfaceCoord {
    coord: DVec2,
}

impl SurfaceCoord {
    pub fn new(u: f64, v: f64) -> SurfaceCoord {
        SurfaceCoord {coord: dvec2!(u.min(1.0).max(0.0), v.min(1.0).max(0.0))}
    }

    pub fn get_u(&self, width: f64) -> u32 {
        assert!(width > 0.0);
        let u = self.coord.x * width;
        let u = (u.floor() as u32).min(width.floor() as u32 - 1);
        u
    }

    pub fn get_v(&self, height: f64) -> u32 {
        assert!(height > 0.0);
        let v = (1.0 - self.coord.y) * height;
        let v = (v.floor() as u32).min(height.floor() as u32 - 1);
        v
    }

    pub fn get_uv(&self, width: f64, height: f64) -> (u32, u32) {
        (self.get_u(width), self.get_v(height))
    }

    pub fn get_coord(&self) -> (f64, f64) {(self.coord.x, self.coord.y)}

}
