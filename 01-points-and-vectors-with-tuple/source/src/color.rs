use crate::vector::Scalar;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<Scalar> for Color {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self {
        Self {
            red: self.red * rhs.v,
            green: self.green * rhs.v,
            blue: self.blue * rhs.v,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Div<Scalar> for Color {
    type Output = Color;

    fn div(self, rhs: Scalar) -> Color {
        Color {
            red: self.red / rhs.v,
            green: self.green / rhs.v,
            blue: self.blue / rhs.v,
        }
    }
}

impl Div<Color> for Color {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            red: self.red / rhs.red,
            green: self.green / rhs.green,
            blue: self.blue / rhs.blue,
        }
    }
}
