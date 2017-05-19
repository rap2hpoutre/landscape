extern crate image;
extern crate imageproc;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;
use image::{ImageBuffer, Rgb, ImageRgb8, PNG, RgbImage};
use imageproc::drawing::draw_filled_circle_mut;

struct Mountain {
    color: Rgb<u8>,
    points: Vec<u32>,
}

impl Mountain {
    fn new(color: Rgb<u8>, y_amp: [u32; 2]) -> Mountain {
        let step_max = 1.0;
        let step_change = 0.25;
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
            points: points
        }
    }
    fn draw(&self, img: &mut RgbImage) {
        let mut i = 0;
        for &point in self.points.iter() {
            img.put_pixel(i, point, self.color);
            for j in point..480 {
                img.put_pixel(i, j, self.color);
            }
            i = i + 1;
        }
    }
}

struct Planet {
    color: Rgb<u8>,
    position: (i32, i32),
    radius: i32
}

impl Planet {
    fn new(color: Rgb<u8>, position: (i32, i32), radius: i32) -> Planet {
        Planet {
            color: color,
            position: position,
            radius: radius,
        }
    }
    fn draw(&self, img: &mut RgbImage) {
        draw_filled_circle_mut(img, (self.position.0, self.position.1), self.radius, self.color);
    }
}

fn color_mix(c1: (u8, u8, u8), c2: (u8, u8, u8), total: u8, step: u8) ->  Rgb<u8> {
    Rgb([
        (c1.0 as i16 + (c2.0 as i16 - c1.0 as i16) / total as i16  * step as i16) as u8,
        (c1.1 as i16 + (c2.1 as i16 - c1.1 as i16) / total as i16  * step as i16) as u8,
        (c1.2 as i16 + (c2.2 as i16 - c1.2 as i16) / total as i16  * step as i16) as u8,
    ])
}

fn main() {
    let mut rng = rand::thread_rng();

    let initial_color = (rng.gen_range(1, 255), rng.gen_range(1, 255), rng.gen_range(1, 255));
    let target_color = match rng.gen_range(1, 4) {
        1 => (rng.gen_range(0,30),rng.gen_range(0,30),rng.gen_range(0,30)),
        2 => (220, 220, rng.gen_range(230, 255)),
        _ => (rng.gen_range(200, 255),rng.gen_range(200, 255),rng.gen_range(200, 255)),
    };
    let planet_color = (rng.gen_range(1, 255), rng.gen_range(1, 255), rng.gen_range(1, 255));

    let mountain_count: u8 = 5;

    let mut img = ImageBuffer::from_pixel(640, 480, Rgb([target_color.0, target_color.1, target_color.2]));

    Planet::new(color_mix(target_color, planet_color, 8, 1), (rng.gen_range(101, 500), rng.gen_range(101, 200)), rng.gen_range(40, 200)).draw(&mut img);

    for i in 0..mountain_count {
        let m = Mountain::new(
            color_mix(target_color, initial_color, mountain_count, (i + 1) as u8),
            [ 400 - 480 / 2 / mountain_count as u32 * (mountain_count as u32 - i as u32), 401 ],
        );
        m.draw(&mut img);
    }

    let ref mut fout = File::create(&Path::new("images/export.png")).unwrap();
    let _ = ImageRgb8(img).save(fout, PNG);
}