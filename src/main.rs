extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;
use image::{ImageBuffer, Rgb, ImageRgb8, PNG, RgbImage};

pub struct Mountain {
    color: Rgb<u8>,
    points: Vec<u32>,
    y_amp: [u32; 2],
}

impl Mountain {
    fn new(color: Rgb<u8>, y_amp: [u32; 2]) -> Mountain {
        let step_max = 1.75;
        let step_change = 0.75;
        let height_max = (y_amp[1] - 1) as f64;
        let height_min = (y_amp[0]) as f64;
        let mut rng = rand::thread_rng();
        let mut height = rng.gen_range(0.0, height_max);
        let mut slope = rng.gen_range(0.0, step_max) * 2.0 - step_max;

        let mut points: Vec<u32> = Vec::new();

        for _ in 0..640 {
            height = height + slope;
            slope = slope + (rng.gen_range(0.0, step_change) * 2.0 - step_change);

            if slope > step_max {
                slope = step_max;
            }
            if slope < -step_max {
                slope = -step_max;
            }

            if height > height_max {
                height = height_max;
                slope = slope * -1.0;
            }
            if height < height_min {
                height = height_min;
                slope = slope * -1.0;
            }
            points.push(height as u32);
        }
        Mountain {
            color: color,
            points: points,
            y_amp: y_amp
        }
    }
    fn draw(&self, img: &mut RgbImage) {
        let mut i = 0;
        for &point in self.points.iter() {
            img.put_pixel(i, point, self.color);
            for j in point..self.y_amp[1] {
                img.put_pixel(i, j, self.color);
            }
            i = i + 1;
        }
    }
}

fn main() {
    let mut img = ImageBuffer::new(640, 480);
    let mut rng = rand::thread_rng();

    let initial_color: (u8, u8, u8) = (
       rng.gen_range(1,255),
       rng.gen_range(1,255), 
       rng.gen_range(1,255)
    );

    let mountain_count: u8 = 5;

    for i in 0..mountain_count {
        
        let m = Mountain::new(
            Rgb([
                initial_color.0 / mountain_count * (i+1) as u8, 
                initial_color.1  / mountain_count * (i+1) as u8, 
                initial_color.2  / mountain_count * (i+1) as u8
            ]), [480 - 480 / 2 / mountain_count as u32 * (mountain_count as u32 - i as u32) , 480]);
        m.draw(&mut img);
    }
    
    let ref mut fout = File::create(&Path::new("images/export.png")).unwrap();
    let _    = ImageRgb8(img).save(fout, PNG);
}
