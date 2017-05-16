extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;
use image::{ImageBuffer, Rgba, ImageRgba8, PNG, RgbaImage};

pub struct Mountain {
    color: Rgba<u8>,
    points: Vec<u32>,
    y_amp: [u8; 2],
}

impl Mountain {
    fn new(color: Rgba<u8>, y_amp: [u8; 2]) -> Mountain {
        let step_max = 1.75;
        let step_change = 0.75;
        let height_max = (y_amp[1] - 1) as f64;
        let height_min = (y_amp[0]) as f64;
        let mut rng = rand::thread_rng();
        let mut height = rng.gen_range(0.0, height_max);
        let mut slope = rng.gen_range(0.0, step_max) * 2.0 - step_max;

        let mut points: Vec<u32> = Vec::new();

        for _ in 0..320 {
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
    fn draw(&self, img: &mut RgbaImage) {
        let mut i = 0;
        for &point in self.points.iter() {
            img.put_pixel(i, point, self.color);
            for j in point..200 {
                img.put_pixel(i, j, self.color);
            }
            i = i + 1;
        }
    }
}

fn main() {
    let mut img = ImageBuffer::new(320, 200);
    
    let m = Mountain::new(Rgba([255, 0, 0, 255]), [20, 200]);
    m.draw(&mut img);
    let m = Mountain::new(Rgba([255, 255, 0, 255]), [100, 200]);
    m.draw(&mut img);
    let m = Mountain::new(Rgba([0, 0, 230, 255]), [140, 180]);
    m.draw(&mut img);
    
    let ref mut fout = File::create(&Path::new("images/export.png")).unwrap();
    let _    = ImageRgba8(img).save(fout, PNG);
}
