use crate::{Point3, Vec3, Transform};

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray{
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn direction_to_point(&self, point: Point3) -> Vec3 {
        point - self.origin
    }

    pub fn transform(&self, transform: &Transform) -> Self {
        Self {
            origin: transform * self.origin,
            direction: transform * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
}