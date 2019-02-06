use euler::DVec3;
use color::Color;

const AMBIENT_PORTION : f64 = 0.1;

#[derive(Clone)]
pub struct AmbientLight {
    pub color: Color,
    pub power: f64,
}

impl AmbientLight {
    pub fn new() -> AmbientLight {
        AmbientLight{color: Color::BLACK, power: 0.0}
    }

    pub fn add_light_source(&mut self, light: &PointLight) {
        self.color = (self.color*self.power + light.color*light.power*AMBIENT_PORTION).normalize();
        self.power += light.power*AMBIENT_PORTION;
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
