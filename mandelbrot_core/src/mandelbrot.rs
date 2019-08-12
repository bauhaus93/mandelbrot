use histogram::Histogram;
use num::Complex;
use palette;
use palette::Pixel;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{snapshot, ColorBucket, MandelbrotError};

const DEFAULT_STEP: f64 = 1. / 800.;
const DEFAULT_COLOR_LOOP: i32 = 100;
// const DEFAULT_POS: [f64; 2] = [0.4379242413594627, -0.3418920843381163];
const DEFAULT_POS: [f64; 2] = [0.41825764120184555, -0.34087020355542164];

pub struct Mandelbrot {
    step_size: f64,
    center: [f64; 2],
    depth: u32,
    color_loop_depth: i32,
    color_buckets: Vec<ColorBucket>,
}

impl Mandelbrot {
    pub fn set_center(&mut self, new_center: [f64; 2]) {
        self.center = new_center;
    }

    pub fn move_center(&mut self, units: [i32; 2]) {
        self.center[0] += self.step_size * units[0] as f64;
        self.center[1] += self.step_size * units[1] as f64;
    }

    pub fn set_step_size(&mut self, value: f64) {
        self.step_size = value;
    }

    pub fn set_step_default(&mut self) {
        self.step_size = DEFAULT_STEP;
    }

    pub fn zoom(&mut self, factor: f64) {
        self.step_size *= factor;
    }

    pub fn set_depth(&mut self, value: u32) {
        self.depth = value;
        self.update_buckets();
    }

    pub fn mod_depth(&mut self, value: u32) {
        self.set_depth(self.get_depth() + value);
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

    pub fn randomize_start_color(&mut self) {
        self.color_buckets.clear();
        self.color_buckets.push(ColorBucket::random_bucket());
        self.update_buckets();
    }

    pub fn print_stats(&self) {
        info!(
            "center = {} + j{}, step size = {}, depth = {}",
            self.center[0], self.center[1], self.step_size, self.depth
        );
    }

    pub fn estimate_entropy(&self, shape: [i32; 2]) -> f32 {
        const EST_SIZE: i32 = 10;
        let mut histogram = Histogram::new();
        for y in -EST_SIZE / 2..EST_SIZE / 2 {
            for x in -EST_SIZE / 2..EST_SIZE / 2 {
                let local_point = [x * shape[0] / EST_SIZE, y * shape[1] / EST_SIZE];
                let abs_point = self.pixel_to_absolute(local_point);
                match check_mandelbrot(&abs_point, self.depth) {
                    Some(v) => histogram.increment(v as u64 + 1).unwrap(),
                    None => histogram.increment(0).unwrap(),
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

    pub fn snapshot(&mut self, file_name: &str, shape: [i32; 2]) -> Result<(), MandelbrotError> {
        let pixels = self.create_pixels(shape);
        snapshot(&pixels, shape, file_name)
    }

    pub fn snapshot_sequence_zoomed(
        &mut self,
        count: usize,
        shape: [i32; 2],
        zoom_factor: f64,
        file_prefix: &str,
    ) -> Result<(), MandelbrotError> {
        for i in 0..count {
            let file_name = format!("{}{:06}", file_prefix, i);
            self.snapshot(&file_name, shape)?;
            self.zoom(zoom_factor);
            if (i + 1) % 10 == 0 {
                info!("Sequence progress: {}/{}", i + 1, count);
            }
        }
        Ok(())
    }

    pub fn create_pixel_triplets(&self, shape: [i32; 2]) -> Vec<[u8; 3]> {
        let values = self.create_values(shape);
        values
            .par_iter()
            .map(|v| colorize(*v, &self.color_buckets))
            .collect()
    }

    pub fn create_pixels(&mut self, shape: [i32; 2]) -> Vec<u8> {
        let values = self.create_values(shape);
        let mut pixels = Vec::new();
        for v in values.iter() {
            let rgb_triple = colorize(*v, &self.color_buckets);
            pixels.extend_from_slice(&rgb_triple);
        }
        pixels
    }

    fn pixel_to_absolute(&self, point: [i32; 2]) -> [f64; 2] {
        [
            self.center[0] + self.step_size * point[0] as f64,
            self.center[1] + self.step_size * point[1] as f64,
        ]
    }

    fn create_values(&self, shape: [i32; 2]) -> Vec<Option<u32>> {
        let mut points: Vec<[f64; 2]> = Vec::new();
        for y in -shape[1] / 2..shape[1] / 2 {
            for x in -shape[0] / 2..shape[0] / 2 {
                let abs_point = self.pixel_to_absolute([x, y]);
                points.push(abs_point);
            }
        }
        let values: Vec<Option<u32>> = points
            .par_iter()
            .map(|p| check_mandelbrot(p, self.depth))
            .collect();
        values
    }

    fn update_buckets(&mut self) {
        for _ in self.color_buckets.len()..self.depth as usize {
            let next_bucket = match self.color_buckets.last() {
                Some(b) => b.next_bucket(self.color_loop_depth),
                None => ColorBucket::random_bucket(),
            };
            self.color_buckets.push(next_bucket);
        }
        debug!("ColorBuckets: {}", self.color_buckets.len());
    }
}

fn colorize(value: Option<u32>, color_buckets: &[ColorBucket]) -> [u8; 3] {
    match value {
        Some(v) => color_buckets[v as usize].get_color(),
        None => [0, 0, 0],
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
        let mut mb = Self {
            step_size: DEFAULT_STEP,
            center: DEFAULT_POS,
            depth: 400,
            color_loop_depth: DEFAULT_COLOR_LOOP,
            color_buckets: Vec::new(),
        };
        mb.update_buckets();
        mb
    }
}
