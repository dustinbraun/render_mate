#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    #[inline]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r,
            g,
            b,
            a,
        }
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Color) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Color) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl std::ops::Div<f32> for Color {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}