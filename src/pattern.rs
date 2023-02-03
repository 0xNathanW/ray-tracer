use std::fmt::Debug;

use crate::{Point3, Colour, Matrix4};
use crate::transform::Transformable;

pub trait Pattern: Transformable + Send + Sync + Debug {
    
    fn colour_at_pattern(&self, point: &Point3) -> Colour;

    fn colour_at(&self, point: &Point3, obj_inverse: &Matrix4) -> Colour {
        let obj_point = obj_inverse.transform_point(point);
        let pattern_point = self.inverse().transform_point(&obj_point);
        self.colour_at_pattern(&pattern_point)
    }
}

#[derive(Debug)]
pub struct Stripes {
    a:          Colour,
    b:          Colour,
    transform:  Matrix4,
    inverse:    Matrix4,
}

impl Stripes {
    pub fn new(a: Colour, b: Colour) -> Self {
        Self {
            a,
            b,
            transform: Matrix4::identity(),
            inverse: Matrix4::identity(),
        }
    }
}

impl Pattern for Stripes {
    fn colour_at_pattern(&self, point: &Point3) -> Colour {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

impl Transformable for Stripes {

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn set_inverse(&mut self, inverse: Matrix4) {
        self.inverse = inverse;
    }

    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn inverse(&self) -> &Matrix4 {
        &self.inverse
    }
}

#[derive(Debug)]
pub struct Gradient {
    a:          Colour,
    b:          Colour,
    transform:  Matrix4,
    inverse:    Matrix4,
}

impl Gradient {
    pub fn new(a: Colour, b: Colour) -> Self {
        Self {
            a,
            b,
            transform: Matrix4::identity(),
            inverse: Matrix4::identity(),
        }
    }
}

impl Pattern for Gradient {
    fn colour_at_pattern(&self, point: &Point3) -> Colour {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();
        self.a + distance * fraction
    }
}

impl Transformable for Gradient {

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn set_inverse(&mut self, inverse: Matrix4) {
        self.inverse = inverse;
    }

    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn inverse(&self) -> &Matrix4 {
        &self.inverse
    }
}

#[derive(Debug)]
pub struct Rings {
    a:          Colour,
    b:          Colour,
    transform:  Matrix4,
    inverse:    Matrix4,
}

impl Rings {
    pub fn new(a: Colour, b: Colour) -> Self {
        Self {
            a,
            b,
            transform: Matrix4::identity(),
            inverse: Matrix4::identity(),
        }
    }
}

impl Pattern for Rings {
    fn colour_at_pattern(&self, point: &Point3) -> Colour {
        if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

impl Transformable for Rings {

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn set_inverse(&mut self, inverse: Matrix4) {
        self.inverse = inverse;
    }

    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn inverse(&self) -> &Matrix4 {
        &self.inverse
    }
}

#[derive(Debug)]
pub struct Checkers {
    a:          Colour,
    b:          Colour,
    transform:  Matrix4,
    inverse:    Matrix4,
}

impl Checkers {
    pub fn new(a: Colour, b: Colour) -> Self {
        Self {
            a,
            b,
            transform: Matrix4::identity(),
            inverse: Matrix4::identity(),
        }
    }
}

impl Pattern for Checkers {
    fn colour_at_pattern(&self, point: &Point3) -> Colour {
        if (point.x.floor() as i32 + point.y.floor() as i32 + point.z.floor() as i32) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

impl Transformable for Checkers {

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn set_inverse(&mut self, inverse: Matrix4) {
        self.inverse = inverse;
    }

    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn inverse(&self) -> &Matrix4 {
        &self.inverse
    }
}
