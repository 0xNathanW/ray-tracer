use crate::vec3::Vec3;
use crate::matrix::Matrix;

pub type Point3 = Vec3;

// Point3 can be translated, thus we add an extra dimension
// to applied matrices.
impl Point3 {
    pub fn transform_point(&self, matrix: &Matrix<4, 4>) -> Vec3 {
        let mut result = Point3::default();
        result.x = self.x * matrix[0][0] + self.y * matrix[1][0] + self.z * matrix[2][0] + matrix[3][0];
        result.y = self.x * matrix[0][1] + self.y * matrix[1][1] + self.z * matrix[2][1] + matrix[3][1];
        result.z = self.x * matrix[0][2] + self.y * matrix[1][2] + self.z * matrix[2][2] + matrix[3][2];
        let w = self.x * matrix[0][3] + self.y * matrix[1][3] + self.z * matrix[2][3] + matrix[3][3];
        if w != 1.0 && w != 0.0 {
            result /= w;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_matrix() {
        let p = Point3 { x: 1.0, y: 2.0, z: 3.0 };
        let m = Matrix::identity();
        let result = p.transform_point(&m);
        assert_eq!(result, p);
    }
}