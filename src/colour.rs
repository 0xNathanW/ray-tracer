use std::ops::{Mul, Add, AddAssign};
use rand::Rng;
use crate::Vec3;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Colour {
    r: f64,
    g: f64,
    b: f64,
}

pub const RED: Colour    = Colour { r: 1.0, g: 0.0, b: 0.0 };
pub const GREEN: Colour  = Colour { r: 0.0, g: 1.0, b: 0.0 };
pub const BLUE: Colour   = Colour { r: 0.0, g: 0.0, b: 1.0 };
pub const WHITE: Colour  = Colour { r: 1.0, g: 1.0, b: 1.0 };
pub const BLACK: Colour  = Colour { r: 0.0, g: 0.0, b: 0.0 };
pub const PINK: Colour   = Colour { r: 1.0, g: 0.0, b: 1.0 };
pub const YELLOW: Colour = Colour { r: 1.0, g: 1.0, b: 0.0 };
pub const CYAN: Colour   = Colour { r: 0.0, g: 1.0, b: 1.0 };
pub const ORANGE: Colour = Colour { r: 1.0, g: 0.5, b: 0.0 };

impl Colour {

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn new_random<R: Rng>(rng: &mut R) -> Self {
        Self {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
        }
    }

    pub fn new_random_range<R: Rng>(min: f64, max: f64, rng: &mut R) -> Self {
        Self {
            r: rng.gen_range(min..max),
            g: rng.gen_range(min..max),
            b: rng.gen_range(min..max),
        }
    }

    pub fn gamma_correct(&mut self, samples: u32) {
        let scale = 1.0 / (samples as f64);
        self.r = (self.r * scale).sqrt();
        self.g = (self.g * scale).sqrt();
        self.b = (self.b * scale).sqrt();
    }
}

impl From<Colour> for Vec3 {
    fn from(colour: Colour) -> Vec3 {
        Vec3::new(colour.r, colour.g, colour.b)
    }
}


impl From<Colour> for Vec<u8> {
    fn from(colour: Colour) -> Vec<u8> {
        vec![
            (256.0 * colour.r.clamp(0.0, 0.999)) as u8,
            (256.0 * colour.g.clamp(0.0, 0.999)) as u8,
            (256.0 * colour.b.clamp(0.0, 0.999)) as u8,
        ]
    }
}

impl Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        Colour {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Colour {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign<Colour> for Colour {
    fn add_assign(&mut self, rhs: Colour) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
