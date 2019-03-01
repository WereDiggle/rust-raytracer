use image::{Rgb};

pub mod consts;
pub mod operator;

// Color needs to be encoded and decoded
// because most image formats don't store color linearly
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
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color {red, green, blue}
    } 

    pub fn from_f64(color_float: f64) -> Color {
        Color {
            red: color_float,
            green: color_float,
            blue: color_float,
        }
    }

    pub fn or_greater(&self, other: Color) -> bool {
        self.red > other.red || self.blue > other.blue || self.green > other.green
    }

    pub fn diff(&self, other: Color) -> f64 {
        (self.red - other.red).abs() +
        (self.blue - other.blue).abs() +
        (self.green - other.green).abs()
    }

    // Clamps color to a displayable range
    // Intermediate Colors might not be in this range for calculation purposes
    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }

    // Keep ratios the same, largest pigment becomes 1.0
    pub fn normalize(&self) -> Color {
        let max = self.red.max(self.green).max(self.blue); 
        Color {
            red: self.red / max,
            green: self.green / max,
            blue: self.blue / max,
        }
    }

    // Convert to type used by the Image Crate to make pngs
    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb {
            data: [(gamma_encode(self.red) * 255.0) as u8,
                   (gamma_encode(self.green) *255.0) as u8,
                   (gamma_encode(self.blue) * 255.0) as u8]
        }
    }

    // Convert from type used by Image Crate
    pub fn from_rgb(rgb: &Rgb<u8>) -> Color {
        Color {
            red: gamma_decode((rgb.data[0] as f64) / 255.0),
            green: gamma_decode((rgb.data[1] as f64) / 255.0),
            blue: gamma_decode((rgb.data[2] as f64) / 255.0),
        }
    }
}
