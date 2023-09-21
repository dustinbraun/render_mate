#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    #[inline]
    pub fn sq_len(self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    #[inline]
    pub fn len(self) -> f32 {
        self.sq_len().sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.len();
        Vec3 {
            x: self.x/len,
            y: self.y/len,
            z: self.z/len,
        }
    }

    #[inline]
    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - n*2.0*n.dot(self)/n.len()
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Vec3 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}