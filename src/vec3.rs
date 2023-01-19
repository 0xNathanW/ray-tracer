use std::ops::{
    Add, AddAssign, 
    Sub, SubAssign, 
    Mul, MulAssign, 
    Div, DivAssign,
    Neg,
};
use rand::Rng;

use crate::matrix::Matrix;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vec3 { 
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Generate Vec3 methods.
impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random() -> Self {
        Self { x: rand::random(), y: rand::random(), z: rand::random() }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self { x: rng.gen_range(min..max), y: rng.gen_range(min..max), z: rng.gen_range(min..max) }
    }

    // Diffuse method 1.
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    // Diffuse method 2.
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().normalise()
    }

    // Diffuse method 3.
    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {   // In the same hemisphere as the normal.
            in_unit_sphere
        } else {
            -in_unit_sphere
        }   
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Self::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                0.0,
            );
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }
}

impl Into<Matrix<1, 3> > for Vec3 {
    fn into(self) -> Matrix<1, 3> {
        Matrix::new([[self.x, self.y, self.z]])
    }
}

impl Into<Matrix<3, 1> > for Vec3 {
    fn into(self) -> Matrix<3, 1> {
        Matrix::new([[self.x], [self.y], [self.z]])
    }
}

// Vector operations.
impl Vec3 {

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Normalise vector to length 1.
    // Use multiplication instead of division for speed.
    pub fn normalise(&self) -> Vec3 {
        let len = self.length();
        if len == 0.0 {
            return *self;
        }
        let inv_len = 1.0 / len;
        Vec3 { x: self.x * inv_len, y: self.y * inv_len, z: self.z * inv_len }
    }

    // Project vector onto another vector.
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Return vector perpendicular to two other vectors.
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 { x: self.y * other.z - self.z * other.y, y: self.z * other.x - self.x * other.z, z: self.x * other.y - self.y * other.x }
    }

    // Return true if the vector is close to zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        (self.x.abs() < S) && (self.y.abs() < S) && (self.z.abs() < S)
    }
}

// Vector//Matrix operations.
// Don't worry about translation, vectors are only used for direction.
impl Vec3 {
    pub fn transform_vec(&self, matrix: &Matrix<3, 3>) -> Vec3 {
        let mut result = Vec3::default();
        result.x = self.x * matrix[0][0] + self.y * matrix[1][0] + self.z * matrix[2][0];
        result.y = self.x * matrix[0][1] + self.y * matrix[1][1] + self.z * matrix[2][1];
        result.z = self.x * matrix[0][2] + self.y * matrix[1][2] + self.z * matrix[2][2];
        result
    }
}

impl Vec3 {
    pub fn reflect(incident: Vec3, unit_normal: Vec3) -> Self {
        incident - 2.0 * incident.dot(unit_normal) * unit_normal
    }

    // Use Snell's law to calculate the refracted ray.
    pub fn refract(incident: Vec3, normal: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = (-incident).dot(normal).min(1.0);
        let r_out_perpendicular = etai_over_etat * (incident + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perpendicular.squared_length()).abs().sqrt() * normal;
        r_out_perpendicular + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { x: self * other.x, y: self * other.y, z: self * other.z }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x / other.x, y: self.y / other.y, z: self.z / other.z }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Vec3 { x: self.x / other.x, y: self.y / other.y, z: self.z / other.z }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 { x: self.x / other, y: self.y / other, z: self.z / other }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Vec3 { x: self.x / other, y: self.y / other, z: self.z / other }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v.length(), 3.7416573867739413);
    }

    #[test]
    fn test_squared_length() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v.squared_length(), 14.0);
    }

    #[test]
    fn test_normalise() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v.normalise(), Vec3 { x: 0.2672612419124244, y: 0.5345224838248488, z: 0.8017837257372732 });
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(v1.dot(v2), 32.0);
    } 

    #[test]
    fn test_cross() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(v1.cross(v2), Vec3 { x: -3.0, y: 6.0, z: -3.0 });
    }

    #[test]
    fn test_near_zero() {
        let v = Vec3 { x: 0.5e-8, y: 0.3e-8, z: 0.0 };
        assert!(v.near_zero());

        let u = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert!(!u.near_zero());
    }

    #[test]
    fn test_add() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(v1 + v2, Vec3 { x: 5.0, y: 7.0, z: 9.0 });
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        v1 += v2;
        assert_eq!(v1, Vec3 { x: 5.0, y: 7.0, z: 9.0 });
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(v1 - v2, Vec3 { x: -3.0, y: -3.0, z: -3.0 });
    }

    #[test]
    fn test_sub_assign() {
        let mut v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        v1 -= v2;
        assert_eq!(v1, Vec3 { x: -3.0, y: -3.0, z: -3.0 });
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(v1 * v2, Vec3 { x: 4.0, y: 10.0, z: 18.0 });
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        v1 *= v2;
        assert_eq!(v1, Vec3 { x: 4.0, y: 10.0, z: 18.0 });
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v * 2.0, Vec3 { x: 2.0, y: 4.0, z: 6.0 });
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        v *= 2.0;
        assert_eq!(v, Vec3 { x: 2.0, y: 4.0, z: 6.0 });
    }

    #[test]
    fn test_div() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(v1 / v2, Vec3 { x: 0.25, y: 0.4, z: 0.5 });
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        v1 /= v2;
        assert_eq!(v1, Vec3 { x: 0.25, y: 0.4, z: 0.5 });
    }

    #[test]
    fn test_div_scalar() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v / 2.0, Vec3 { x: 0.5, y: 1.0, z: 1.5 });
    }

    #[test]
    fn test_div_assign_scalar() {
        let mut v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        v /= 2.0;
        assert_eq!(v, Vec3 { x: 0.5, y: 1.0, z: 1.5 });
    }

    #[test]
    fn test_neg() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(-v, Vec3 { x: -1.0, y: -2.0, z: -3.0 });
    }
}