use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use super::Color;

// Operation traits for color
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

impl MulAssign for Color {
    fn mul_assign(&mut self, other: Color) {
        *self = Color {
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

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Color {
        Color {
            red: self.red / other,
            blue: self.blue / other,
            green: self.green / other,
        }
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, other: f64) {
        *self = Color {
            red: self.red / other,
            blue: self.blue / other,
            green: self.green / other,
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, other: f64) {
        *self = Color {
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

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            red: self.red - other.red,
            blue: self.blue - other.blue,
            green: self.green - other.green,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        *self = Color {
            red: self.red - other.red,
            blue: self.blue - other.blue,
            green: self.green - other.green,
        }
    }
}