use std::ops;

use crate::linalg::Interval;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn gamma_transform(&self) -> Self {
        Color::new(
            gamma_transform(self.r),
            gamma_transform(self.g),
            gamma_transform(self.b),
        )
    }

    pub fn to_bytes(self) -> (u8, u8, u8) {
        let intensity = Interval::new(0.0, 0.999);

        (
            (256.0 * intensity.clamp(self.r)).floor() as u8,
            (256.0 * intensity.clamp(self.g)).floor() as u8,
            (256.0 * intensity.clamp(self.b)).floor() as u8,
        )
    }
}

fn gamma_transform(x: f64) -> f64 {
    if x > 0.0 { x.sqrt() } else { 0.0 }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
