use crate::Vec3;
use crate::Vec4;

#[derive(Copy, Clone)]
pub struct Mat4(
    (f32, f32, f32, f32),
    (f32, f32, f32, f32),
    (f32, f32, f32, f32),
    (f32, f32, f32, f32),
);

impl Mat4 {
    pub fn new_identity() -> Mat4 {
        Mat4(
            (1.0, 0.0, 0.0, 0.0),
            (0.0, 1.0, 0.0, 0.0),
            (0.0, 0.0, 1.0, 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn new_translation(translation: Vec3) -> Mat4 {
        Mat4(
            (1.0, 0.0, 0.0, translation.x),
            (0.0, 1.0, 0.0, translation.y),
            (0.0, 0.0, 1.0, translation.z),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn new_scale(scale: Vec3) -> Mat4 {
        Mat4(
            (scale.x, 0.0, 0.0, 0.0),
            (0.0, scale.y, 0.0, 0.0),
            (0.0, 0.0, scale.z, 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn new_rotation_x(rotation: f32) -> Mat4 {
        Mat4(
            (1.0, 0.0, 0.0, 0.0),
            (0.0, rotation.cos(), -(rotation.sin()), 0.0),
            (0.0, rotation.sin(), rotation.cos(), 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn new_rotation_y(rotation: f32) -> Mat4 {
        Mat4(
            (rotation.cos(), 0.0, rotation.sin(), 0.0),
            (0.0, 1.0, 0.0, 0.0),
            (-(rotation.sin()), 0.0, rotation.cos(), 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn new_rotation_z(rotation: f32) -> Mat4 {
        Mat4(
            (rotation.cos(), -(rotation.sin()), 0.0, 0.0),
            (rotation.sin(), rotation.cos(), 0.0, 0.0),
            (0.0, 0.0, 1.0, 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }
}

impl std::ops::Add for Mat4 {
    type Output = Mat4;

    fn add(self, rhs: Mat4) -> Mat4 {
        Mat4(
            (self.0.0 + rhs.0.0, self.0.1 + rhs.0.1, self.0.2 + rhs.0.2, self.0.3 + rhs.0.3),
            (self.1.0 + rhs.1.0, self.1.1 + rhs.1.1, self.1.2 + rhs.1.2, self.1.3 + rhs.1.3),
            (self.2.0 + rhs.2.0, self.2.1 + rhs.2.1, self.2.2 + rhs.2.2, self.2.3 + rhs.2.3),
            (self.3.0 + rhs.3.0, self.3.1 + rhs.3.1, self.3.2 + rhs.3.2, self.3.3 + rhs.3.3),
        )
    }
}

impl std::ops::Sub for Mat4 {
    type Output = Mat4;

    fn sub(self, rhs: Mat4) -> Mat4 {
        Mat4(
            (self.0.0 - rhs.0.0, self.0.1 - rhs.0.1, self.0.2 - rhs.0.2, self.0.3 - rhs.0.3),
            (self.1.0 - rhs.1.0, self.1.1 - rhs.1.1, self.1.2 - rhs.1.2, self.1.3 - rhs.1.3),
            (self.2.0 - rhs.2.0, self.2.1 - rhs.2.1, self.2.2 - rhs.2.2, self.2.3 - rhs.2.3),
            (self.3.0 - rhs.3.0, self.3.1 - rhs.3.1, self.3.2 - rhs.3.2, self.3.3 - rhs.3.3),
        )
    }
}

impl std::ops::Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4(
            (self.0.0*rhs.0.0 + self.0.1*rhs.1.0 + self.0.2*rhs.2.0 + self.0.3*rhs.3.0, self.0.0*rhs.0.1 + self.0.1*rhs.1.1 + self.0.2*rhs.2.1 + self.0.3*rhs.3.1, self.0.0*rhs.0.2 + self.0.1*rhs.1.2 + self.0.2*rhs.2.2 + self.0.3*rhs.3.2, self.0.0*rhs.0.3 + self.0.1*rhs.1.3 + self.0.2*rhs.2.3 + self.0.3*rhs.3.3),
            (self.1.0*rhs.0.0 + self.1.1*rhs.1.0 + self.1.2*rhs.2.0 + self.1.3*rhs.3.0, self.1.0*rhs.0.1 + self.1.1*rhs.1.1 + self.1.2*rhs.2.1 + self.1.3*rhs.3.1, self.1.0*rhs.0.2 + self.1.1*rhs.1.2 + self.1.2*rhs.2.2 + self.1.3*rhs.3.2, self.1.0*rhs.0.3 + self.1.1*rhs.1.3 + self.1.2*rhs.2.3 + self.1.3*rhs.3.3),
            (self.2.0*rhs.0.0 + self.2.1*rhs.1.0 + self.2.2*rhs.2.0 + self.2.3*rhs.3.0, self.2.0*rhs.0.1 + self.2.1*rhs.1.1 + self.2.2*rhs.2.1 + self.2.3*rhs.3.1, self.2.0*rhs.0.2 + self.2.1*rhs.1.2 + self.2.2*rhs.2.2 + self.2.3*rhs.3.2, self.2.0*rhs.0.3 + self.2.1*rhs.1.3 + self.2.2*rhs.2.3 + self.2.3*rhs.3.3),
            (self.3.0*rhs.0.0 + self.3.1*rhs.1.0 + self.3.2*rhs.2.0 + self.3.3*rhs.3.0, self.3.0*rhs.0.1 + self.3.1*rhs.1.1 + self.3.2*rhs.2.1 + self.3.3*rhs.3.1, self.3.0*rhs.0.2 + self.3.1*rhs.1.2 + self.3.2*rhs.2.2 + self.3.3*rhs.3.2, self.3.0*rhs.0.3 + self.3.1*rhs.1.3 + self.3.2*rhs.2.3 + self.3.3*rhs.3.3),
        )
    }
}

impl std::ops::Mul<f32> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: f32) -> Mat4 {
        Mat4(
            (self.0.0*rhs, self.0.1*rhs, self.0.2*rhs, self.0.3*rhs),
            (self.1.0*rhs, self.1.1*rhs, self.1.2*rhs, self.1.3*rhs),
            (self.2.0*rhs, self.2.1*rhs, self.2.2*rhs, self.2.3*rhs),
            (self.3.0*rhs, self.3.1*rhs, self.3.2*rhs, self.3.3*rhs),
        )
    }
}

impl std::ops::Div<f32> for Mat4 {
    type Output = Mat4;

    fn div(self, rhs: f32) -> Mat4 {
        let inv = 1.0/rhs;
        Mat4(
            (self.0.0*inv, self.0.1*inv, self.0.2*inv, self.0.3*inv),
            (self.1.0*inv, self.1.1*inv, self.1.2*inv, self.1.3*inv),
            (self.2.0*inv, self.2.1*inv, self.2.2*inv, self.2.3*inv),
            (self.3.0*inv, self.3.1*inv, self.3.2*inv, self.3.3*inv),
        )
    }
}

impl std::ops::AddAssign for Mat4 {
    fn add_assign(&mut self, rhs: Mat4) {
        self.0.0 += rhs.0.0;
        self.0.1 += rhs.0.1;
        self.0.2 += rhs.0.2;
        self.0.3 += rhs.0.3;
        self.1.0 += rhs.1.0;
        self.1.1 += rhs.1.1;
        self.1.2 += rhs.1.2;
        self.1.3 += rhs.1.3;
        self.2.0 += rhs.2.0;
        self.2.1 += rhs.2.1;
        self.2.2 += rhs.2.2;
        self.2.3 += rhs.2.3;
        self.3.0 += rhs.3.0;
        self.3.1 += rhs.3.1;
        self.3.2 += rhs.3.2;
        self.3.3 += rhs.3.3;
    }
}

impl std::ops::SubAssign for Mat4 {
    fn sub_assign(&mut self, rhs: Mat4) {
        self.0.0 -= rhs.0.0;
        self.0.1 -= rhs.0.1;
        self.0.2 -= rhs.0.2;
        self.0.3 -= rhs.0.3;
        self.1.0 -= rhs.1.0;
        self.1.1 -= rhs.1.1;
        self.1.2 -= rhs.1.2;
        self.1.3 -= rhs.1.3;
        self.2.0 -= rhs.2.0;
        self.2.1 -= rhs.2.1;
        self.2.2 -= rhs.2.2;
        self.2.3 -= rhs.2.3;
        self.3.0 -= rhs.3.0;
        self.3.1 -= rhs.3.1;
        self.3.2 -= rhs.3.2;
        self.3.3 -= rhs.3.3;
    }
}

impl std::ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4::new(
            self.0.0*rhs.x + self.0.1*rhs.y + self.0.2*rhs.z + self.0.3*rhs.w,
            self.1.0*rhs.x + self.1.1*rhs.y + self.1.2*rhs.z + self.1.3*rhs.w,
            self.2.0*rhs.x + self.2.1*rhs.y + self.2.2*rhs.z + self.2.3*rhs.w,
            self.3.0*rhs.x + self.3.1*rhs.y + self.3.2*rhs.z + self.3.3*rhs.w,
        )
    }
}

impl std::ops::Mul<Mat4> for f32 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        rhs*self
    }
}

impl std::ops::Div<Mat4> for f32 {
    type Output = Mat4;

    fn div(self, rhs: Mat4) -> Mat4 {
        rhs/self
    }
}