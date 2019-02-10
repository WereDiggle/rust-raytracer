use euler::DVec3;
use color::Color;

const AMBIENT_PORTION : f64 = 0.01;

#[derive(Clone)]
pub struct AmbientLight {
    pub color: Color,
    pub power: f64,
}

impl AmbientLight {
    pub fn new(color: Color, power: f64) -> AmbientLight {
        AmbientLight{color, power}
    }

    pub fn color_intensity(&self) -> Color {
        self.color * self.power
    }
}

#[derive(Clone)]
pub struct PointLight {
    pub position: DVec3,
    pub color: Color,
    pub power: f64,
    pub falloff: (f64, f64, f64),
}

impl PointLight {
    pub fn new(position: DVec3, color: Color, power: f64, falloff: (f64, f64, f64)) -> PointLight {
        PointLight{position, color: color.normalize(), power, falloff}
    }

    pub fn color_intensity(&self) -> Color {
        self.color * self.power
    }
}
