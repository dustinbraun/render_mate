#[derive(Copy, Clone, Debug)]
pub struct Extent {
    pub width: u32,
    pub height: u32,
}

impl Extent {
    #[inline]
    pub fn new(width: u32, height: u32) -> Extent {
        Extent {
            width,
            height,
        }
    }
}