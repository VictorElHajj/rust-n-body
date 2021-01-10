use std::ops::{Add, Div, Mul, Sub};

const EPSILON: f64 = 0.001;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    pub fn zero() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }

    pub fn distance(&self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + EPSILON).sqrt()
    }

    pub fn distance_2(&self, other: Self) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + EPSILON
    }

    pub fn normal_vector_between(&self, other: Self) -> Self {
        (other - *self) / other.distance(*self)
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f64> for Vector2 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Vector2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
