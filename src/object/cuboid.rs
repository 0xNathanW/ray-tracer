use crate::Point3;
use crate::Matrix;

pub struct AxisAlignedBoundingBox {
    min: Point3,
    max: Point3,
}





pub struct OrientatedBoundingBox {
    min: Point3,
    max: Point3,
    rotation: Matrix<3, 3>,
}
