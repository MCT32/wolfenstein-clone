use std::ops::{ Add, Mul };
use std::fmt;

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn to_u32(&self) -> u32 {
        self.blue as u32 | (self.green as u32) << 8 | (self.red as u32) << 16
    }

    pub fn interp(v1: Color, v2: Color, fac: f32) -> Color {
        let fac = fac.clamp(0.0, 1.0);
        v1 * (1.0 - fac) + v2 * fac
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            red: (self.red + other.red).clamp(0, 255),
            green: (self.green + other.red).clamp(0, 255),
            blue: (self.blue + other.blue).clamp(0, 255),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            red: (self.red as f32 * rhs) as u8,
            green: (self.green as f32 * rhs) as u8,
            blue: (self.blue as f32 * rhs) as u8,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.red, self.green, self.blue)
    }
}