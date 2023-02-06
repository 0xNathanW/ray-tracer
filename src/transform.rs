use crate::{Matrix4, Axis, Vec3, Rotation, Translation, Scale};

pub trait Transformable {
    
    fn transform(&self) -> &Matrix4;

    fn set_transform(&mut self, transform: Matrix4);

    fn inverse(&self) -> &Matrix4;

    fn set_inverse(&mut self, inverse: Matrix4);

    fn rotate(&mut self, axis: Axis, angle: f64) {
        let angle = angle.to_radians();
        let rotation = match axis {
            Axis::X => Rotation::from_axis_angle(&Vec3::x_axis(), angle),
            Axis::Y => Rotation::from_axis_angle(&Vec3::y_axis(), angle),
            Axis::Z => Rotation::from_axis_angle(&Vec3::z_axis(), angle),
        }.to_homogeneous();
        
        let inv = rotation.try_inverse().expect("Rotation matrix is not invertible.");
        self.set_transform(self.transform() * rotation);
        self.set_inverse(inv * self.inverse());
    }

    fn translate(&mut self, x: f64, y: f64, z: f64) {
        let translation = Translation::new(x, y, z).to_homogeneous();
        self.set_transform(self.transform() * translation);
        
        let inv = translation.try_inverse().expect("Translation matrix is not invertible.");
        self.set_inverse(inv * self.inverse());
    }

    fn scale(&mut self, x: f64, y: f64, z: f64) {
        let scale = Scale::new(x, y, z).to_homogeneous();
        let inv = scale.try_inverse().expect("Scale matrix is not invertible.");

        self.set_transform(self.transform() * scale);
        self.set_inverse(inv * self.inverse());
    }

    fn scale_uniform(&mut self, scale: f64) {
        self.scale(scale, scale, scale);
    }

    fn shear(&mut self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) {
        let shear = Matrix4::new(
            1.0, xy, xz, 0.0,
            yx, 1.0, yz, 0.0,
            zx, zy, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        let inv = shear.try_inverse().expect("Shear matrix is not invertible.");

        self.set_transform(self.transform() * shear);
        self.set_inverse(inv * self.inverse());
    }
}