use crate::Mat4;
use crate::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Mat3(
    pub (f32, f32, f32),
    pub (f32, f32, f32),
    pub (f32, f32, f32),
);

impl From<Mat4> for Mat3 {
    fn from(value: Mat4) -> Mat3 {
        Mat3(
            (value.0.0, value.0.1, value.0.2),
            (value.1.0, value.1.1, value.1.2),
            (value.2.0, value.2.1, value.2.2),
        )
    }
}

impl std::ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.0.0*rhs.x + self.0.1*rhs.y + self.0.2*rhs.z,
            self.1.0*rhs.x + self.1.1*rhs.y + self.1.2*rhs.z,
            self.2.0*rhs.x + self.2.1*rhs.y + self.2.2*rhs.z,
        )
    }
}