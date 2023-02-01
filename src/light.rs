use crate::Point3;
use crate::colour::Colour;

pub struct Light {
    pub position: Point3,
    pub intensity: Colour,
}

impl Light {
    pub fn new(position: Point3, intensity: Colour) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

