extern crate image;
extern crate imageproc;
extern crate rand;

use rand::{Rng, ThreadRng};
use std::fs::File;
use std::path::Path;
use image::{ImageBuffer, Rgb, ImageRgb8, PNG, RgbImage};
use imageproc::drawing::draw_filled_circle_mut;
use imageproc::pixelops::interpolate;

struct Mountain {
    color: Rgb<u8>,
    points: Vec<u32>,
}

impl Mountain {
    fn new(color: Rgb<u8>, y_amp: [u32; 2]) -> Mountain {
        let mut rng = rand::thread_rng();
        let step_max = rng.gen_range(0.9, 1.1);
        let step_change = rng.gen_range(0.15, 0.35);
        let height_max = (y_amp[1] - 1) as f64;
        let height_min = (y_amp[0]) as f64;
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

fn rgb_rand(rng: &mut ThreadRng, r: (u8, u8), g: (u8, u8), b: (u8, u8)) -> Rgb<u8> {
    Rgb([rng.gen_range(r.0, r.1), rng.gen_range(g.0, g.1), rng.gen_range(b.0, b.1)])
}

fn main() {
    let mut rng = rand::thread_rng();
    let c_sky = match rng.gen_range(1, 4) {
        1 => rgb_rand(&mut rng, (1, 40), (1, 40), (1, 40)),
        2 => rgb_rand(&mut rng, (215, 225), (215, 225), (230, 255)),
        _ => rgb_rand(&mut rng, (200, 255), (200, 255), (200, 255)),
    };
    let mut img = ImageBuffer::from_pixel(640, 480, c_sky);

    if rng.gen_weighted_bool(2) {
        let x = rng.gen_range(101, 520);
        let y = rng.gen_range(81, 200);
        let rad = rng.gen_range(20, 80);
        let c_planet = interpolate(rgb_rand(&mut rng, (1, 255), (1, 255), (1, 255)), c_sky, 0.1);
        draw_filled_circle_mut(&mut img, (x, y), rad, c_planet);
        if rng.gen_weighted_bool(2) {
            draw_filled_circle_mut(&mut img, (x + rng.gen_range(-2, 4) * 10, y), rad, c_sky);
        }
    }
    
    let mountain_count: u32 = rng.gen_range(4, 7);
    let c_mountain = rgb_rand(&mut rng, (1, 255), (1, 255), (1, 255));
    for i in 0..mountain_count {
        let c = interpolate(c_mountain, c_sky, (i + 1) as f32 / mountain_count as f32);
        let y_amp = [ 400 - 480 / 2 / mountain_count * (mountain_count - i), 401 ];
        Mountain::new(c, y_amp).draw(&mut img);
    }

    let ref mut fout = File::create(&Path::new("images/export.png")).unwrap();
    let _ = ImageRgb8(img).save(fout, PNG);
}