use palette;
use palette::Pixel;
use num::Complex;
use rayon::iter::{ ParallelIterator, IntoParallelRefIterator };
use histogram::Histogram;

use crate::{ MandelbrotError, snapshot };

pub struct Mandelbrot {
    step_size: f64,
    center: [f64; 2],
    depth: u32
}

impl Mandelbrot {
    pub fn set_center(&mut self, new_center: [f64; 2]) {
        self.center = new_center;
    }

    pub fn move_center(&mut self, percent: [f64; 2], shape: [i32; 2]) {
        self.center[0] += self.step_size * percent[0] * shape[0] as f64;
        self.center[1] += self.step_size * percent[1] * shape[1] as f64;
    }

    pub fn set_step_size(&mut self, value: f64) {
        self.step_size = value;
    }
    
    pub fn zoom(&mut self, factor: f64) {
        self.step_size *= factor;
    }

    pub fn set_depth(&mut self, value: u32) {
        self.depth = value;
    }

    pub fn mod_depth(&mut self, value: u32) {
        self.depth += value;
    }

    pub fn get_center(&self) -> [f64; 2] {
        self.center
    }
    pub fn get_step_size(&self) -> f64 {
        self.step_size
    }
    pub fn get_depth(&self) -> u32 {
        self.depth
    }

    pub fn print_stats(&self) {
        info!("center = {} + j{}, step size = {}, depth = {}",
            self.center[0],
            self.center[1],
            self.step_size,
            self.depth);
    }

    pub fn estimate_entropy(&self, shape: [i32;2]) -> f32 {
        const EST_SIZE: i32 = 10;
        let mut histogram = Histogram::new();
        for y in -EST_SIZE / 2..EST_SIZE / 2 {
            for x in -EST_SIZE / 2..EST_SIZE / 2 {
                let local_point = [x * shape[0] / EST_SIZE,
                                   y * shape[1] / EST_SIZE];
                let abs_point = self.pixel_to_absolute(local_point);
                match check_mandelbrot(&abs_point, self.depth) {
                    Some(v) => histogram.increment(v as u64 + 1).unwrap(),
                    None => histogram.increment(0).unwrap()
                }
            }
        }

        let total = histogram.entries();
        let mut entropy = 0.;
        for bucket in histogram.into_iter() {
            let prob = bucket.count() as f32 / total as f32;
            if prob > 0. {
                entropy += prob * prob.log2();
            }
        }
        -entropy
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

    fn pixel_to_absolute(&self, point: [i32; 2]) -> [f64; 2] {
        [self.center[0] + self.step_size * point[0] as f64,
         self.center[1] + self.step_size * point[1] as f64]
    }

    fn create_values(&self, shape: [i32; 2]) -> (Vec<Option<u32>>, u32, u32) {
        let mut points: Vec<[f64; 2]> = Vec::new();
        for y in -shape[1] / 2..shape[1] / 2 {
            for x in -shape[0] / 2..shape[0] / 2 {
                let abs_point = self.pixel_to_absolute([x, y]);
                points.push(abs_point); 
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
