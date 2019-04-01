use euler::{DVec2, dvec2};

#[derive(Clone, Copy)]
pub struct SurfaceCoord {
    coord: DVec2,
}

impl SurfaceCoord {
    pub fn new(u: f64, v: f64) -> SurfaceCoord {
        use std::f64::EPSILON as e;
        // Letting uv get to 1.0 is actually too much of a hassle, so bind it to [0.0, 1.0)
        SurfaceCoord {coord: dvec2!(u.min(1.0-e).max(0.0), v.min(1.0-e).max(0.0))}
    }

    pub fn get_u_index(&self, width: u32) -> u32 {
        assert!(width > 0);
        (self.coord.x * width as f64).floor() as u32
    }

    pub fn get_v_index(&self, height: u32) -> u32 {
        assert!(height > 0);
        (self.coord.y * height as f64).floor() as u32
    }

    pub fn get_uv_index(&self, width: u32, height: u32) -> (u32, u32) {
        (self.get_u_index(width), self.get_v_index(height))
    }

    pub fn get_u_decimal(&self, width: u32) -> f64 {
        assert!(width > 0);
        (self.coord.x * width as f64).fract()
    }

    pub fn get_v_decimal(&self, height: u32) -> f64 {
        assert!(height > 0);
        (self.coord.y * height as f64).fract()
    }

    pub fn get_uv_decimal(&self, width: u32, height: u32) -> (f64, f64) {
        (self.get_u_decimal(width), self.get_v_decimal(height))
    }

    pub fn get_coord(&self) -> (f64, f64) {(self.coord.x, self.coord.y)}

}
