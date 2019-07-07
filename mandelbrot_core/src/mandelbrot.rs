use palette;
use palette::Pixel;
use num::Complex;
use rayon::iter::{ ParallelIterator, IntoParallelRefIterator };

use crate::{ MandelbrotError, snapshot };

pub struct Mandelbrot {
    step_size: f64,
    center: [f64; 2],
    depth: u32
}

impl Mandelbrot {

    pub fn move_center(&mut self, percent: [f64; 2], shape: [i32; 2]) {
        self.center[0] += self.step_size * percent[0] * shape[0] as f64;
        self.center[1] += self.step_size * percent[1] * shape[1] as f64;
    }
    
    pub fn zoom(&mut self, factor: f64) {
        self.step_size *= factor;
    }

    pub fn mod_depth(&mut self, value: u32) {
        self.depth += value;
    }

    pub fn print_stats(&self) {
        info!("center = {} + j{}, step size = {}, depth = {}",
            self.center[0],
            self.center[1],
            self.step_size,
            self.depth);
    }

    pub fn snapshot(&self, file_name: &str, shape: [i32; 2]) -> Result<(), MandelbrotError> {
        let pixels = self.create_pixels(shape);
        snapshot(&pixels, shape, file_name)
    }

    pub fn create_pixel_triplets(&self, shape: [i32; 2]) -> Vec<[u8; 3]> {
       let (values, min, max) = self.create_values(shape); 
        values.par_iter()
            .map(|v| colorize(*v, min, max))
            .collect()
    }

    pub fn create_pixels(&self, shape: [i32; 2]) -> Vec<u8> {
        let (values, min, max) = self.create_values(shape);
        let mut pixels = Vec::new();
        for v in values.iter() {
            let rgb_triple = colorize(*v, min, max);
            pixels.extend_from_slice(&rgb_triple);
        }
        pixels
    }

    fn create_values(&self, shape: [i32; 2]) -> (Vec<Option<u32>>, u32, u32) {
        let mut points: Vec<[f64; 2]> = Vec::new();
        for y in -shape[1] / 2..shape[1] / 2 {
            for x in -shape[0] / 2..shape[0] / 2 {
                let point = [self.center[0] + self.step_size * x as f64,
                             self.center[1] + self.step_size * y as f64];
                points.push(point); 
            }
        }
        let values: Vec<Option<u32>> = points.par_iter()
            .map(|p| check_mandelbrot(p, self.depth))
            .collect();
        let min = values.iter().fold(u32::max_value(), |curr_min, v| match v {
                Some(n) => u32::min(curr_min, *n),
                None => curr_min
        });

        let max = values.iter().fold(u32::min_value(), |curr_max, v| match v {
                Some(n) => u32::max(curr_max, *n),
                None => curr_max
        });
        (values, min, max)
    }

}

fn colorize(value: Option<u32>, min: u32, max: u32) -> [u8; 3] {
    match value {
        Some(v) => {
            let perc = (v - min) as f32 / max as f32;
            let hue = 360. * perc;
            let rgb_f: [f32; 3] = palette::Srgb::from(palette::Hsv::new(hue, 1., 1.)).into_raw();
            [(rgb_f[0] * 255.) as u8, (rgb_f[1] * 255.) as u8, (rgb_f[2] * 255.) as u8]
        },
        None => [0, 0, 0]
    }
}

fn check_mandelbrot(point: &[f64; 2], max_depth: u32) -> Option<u32> {
    let c = Complex::new(point[0], point[1]);
    let mut z = Complex::new(0., 0.);
    for i in 0..max_depth {
        z = z.powf(2.) + c;
        if z.norm() >= 2. {
            return Some(i);
        }
    }
    None
}


impl Default for Mandelbrot {
    fn default() -> Self {
        Self {
            step_size: 1. / 800.,
            center: [0.5, 0.],
            depth: 255
        }
    }
}
