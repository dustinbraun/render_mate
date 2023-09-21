use crate::Color;
use crate::Extent;

pub struct Framebuffer {
    extent: Extent,
    data: Vec<Color>,
}

impl Framebuffer {
    pub fn new(extent: Extent) -> Framebuffer {
        let mut data: Vec<Color> = vec![];
        for _ in 0..extent.height {
            for _ in 0..extent.width {
                data.push(Color::new(
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                ));
            }
        }
        Framebuffer {
            extent,
            data,
        }
    }

    #[inline]
    pub fn get_extent(&self) -> Extent {
        self.extent
    }

    #[inline]
    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.data[(y*self.extent.width + x) as usize] = color;
    }

    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.data[(y*self.extent.width + x) as usize]
    }

    pub fn merge(&mut self, other: &Framebuffer) {
        for y in 0..self.extent.height {
            for x in 0..self.extent.width {
                self.data[(y*self.extent.width + x) as usize] = (self.data[(y*self.extent.width + x) as usize] + other.data[(y*self.extent.width + x) as usize])*0.5;
            }
        }
    }

    pub fn save(&self, path: &str) {
        let mut image = image::RgbImage::new(self.extent.width, self.extent.height);
        for y in 0..self.extent.height {
            for x in 0..self.extent.width {
                let color = self.get_pixel(x, y);
                let r = (255.0*color.r).clamp(0.0, 255.0) as u8;
                let g = (255.0*color.g).clamp(0.0, 255.0) as u8;
                let b = (255.0*color.b).clamp(0.0, 255.0) as u8;
                // let a = (255.0*color.a).clamp(0.0, 255.0) as u8;
                image.put_pixel(x, y, image::Rgb([r, g, b]));
            }
        }
        image.save(path).unwrap();
    }
}