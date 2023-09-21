
#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[inline]
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 {
            x,
            y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x*rhs,
            y: self.y*rhs,
        }
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x/rhs,
            y: self.y/rhs,
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}