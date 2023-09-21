
use crate::Color;
use crate::Extent;
use crate::Vec2;

pub struct Texture {
    extent: Extent,
    data: Vec<Color>,
}

impl Texture {
    pub fn from(path: &str) -> Texture {
        let image = image::open(path).unwrap().into_rgba32f();
        let extent = Extent {
            width: image.width(),
            height: image.height(),
        };
        let mut data: Vec<Color> = vec![];
        for y in 0..extent.height {
            for x in 0..extent.width {
                let c = Color {
                    r: image.get_pixel(x, y).0[0],
                    g: image.get_pixel(x, y).0[1],
                    b: image.get_pixel(x, y).0[2],
                    a: image.get_pixel(x, y).0[3],
                };
                data.push(c);
            }
        }
        Texture {
            extent,
            data,
        }
    }

    pub fn sample(&self, texture_coords: Vec2) -> Color {
        let x = (texture_coords.x*(self.extent.width as f32)).clamp(0.0, (self.extent.width - 1) as f32) as usize;
        let y = (texture_coords.y*(self.extent.height as f32)).clamp(0.0, (self.extent.height - 1) as f32) as usize;
        self.data[y*self.extent.width as usize + x]
    }
}