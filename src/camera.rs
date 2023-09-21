use rand::Rng;

use crate::Extent;
use crate::Ray;
use crate::Vec3;

pub struct Camera {
    frame_extent: Extent,
}

impl Camera {
    pub fn new(frame_extent: Extent) -> Camera {
        Camera {
            frame_extent
        }
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        let rand_x = rand::thread_rng().gen::<f32>()  - 0.5;
        let rand_y = rand::thread_rng().gen::<f32>()  - 0.5;
        let x = -1.0 + (x as f32) * ( 2.0 / (((self.frame_extent.width - 1) as f32 + rand_x)));
        let y =  1.0 - (y as f32) * ( 2.0 / (((self.frame_extent.height - 1) as f32 + rand_y)));
        let point_on_plane = Vec3::new(x, y, 0.0);
        let ray_origin = Vec3::new(0.0, 0.0, -1.0);
        let ray_direction = (point_on_plane - ray_origin).normalize();
        Ray {
            origin: ray_origin,
            direction: ray_direction,
            t_min: 0.0,
            t_max: 10000.0,
        }
    }
}