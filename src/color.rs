use std::ops::{Add, AddAssign, Mul};
use image::{Rgb};

const GAMMA: f64 = 2.2;

pub fn gamma_encode(linear: f64) -> f64 {
    linear.powf(1.0 / GAMMA)
}

pub fn gamma_decode(encoded: f64) -> f64 {
    encoded.powf(GAMMA)
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {

    pub const BLACK: Color = Color { red: 0.0, green: 0.0, blue: 0.0 };
    pub const WHITE: Color = Color { red: 1.0, green: 1.0, blue: 1.0 };
    pub const RED: Color   = Color { red: 1.0, green: 0.0, blue: 0.0 };
    pub const GREEN: Color = Color { red: 0.0, green: 1.0, blue: 0.0 };
    pub const BLUE: Color  = Color { red: 0.0, green: 0.0, blue: 1.0 };

    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color {red, green, blue}
    } 


    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }

    pub fn normalize(&self) -> Color {
        let max = self.red.max(self.green).max(self.blue); 
        Color {
            red: self.red / max,
            green: self.green / max,
            blue: self.blue / max,
        }
    }

    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb {
            data: [(gamma_encode(self.red) * 255.0) as u8,
                   (gamma_encode(self.green) *255.0) as u8,
                   (gamma_encode(self.blue) * 255.0) as u8]
        }
    }

    pub fn from_rgb(rgb: Rgb<u8>) -> Color {
        Color {
            red: gamma_decode((rgb.data[0] as f64) / 255.0),
            green: gamma_decode((rgb.data[1] as f64) / 255.0),
            blue: gamma_decode((rgb.data[2] as f64) / 255.0),
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}