
use num::Complex;
use rayon::iter::{ ParallelIterator, IntoParallelRefIterator };

pub struct Mandelbrot {
    shape: [i32; 2],
    step_size: f64,
    center: [f64; 2],
    depth: u32
}

fn hsv_to_rgb(hsv: [f32; 3]) -> [u8; 3] {
    let c = hsv[1] * hsv[1];
    let h = hsv[0] * 60.;
    let x = c * (1 - (h as i32 % 2 - 1)).abs() as f32;
    let rgb = match h {
        h if h <= 1. => [c, x, 0.],
        h if h <= 2. => [x, c, 0.],
        h if h <= 3. => [0., c, x],
        h if h <= 4. => [0., x, c],
        h if h <= 5. => [x, 0., c],
        h if h <= 6. => [c, 0., x],
        _ => [0., 0., 0.]
    };
    let m = hsv[2] - c;
    [(rgb[0] * m) as u8,
     (rgb[1] * m) as u8,
     (rgb[2] * m) as u8]
}

impl Mandelbrot {

    pub fn set_shape(&mut self, new_shape: [i32; 2]) {
        self.shape = new_shape;
    }

    pub fn get_shape(&self) -> [i32; 2] {
        self.shape
    }

    pub fn move_center(&mut self, percent: [f64; 2]) {
        self.center[0] += self.step_size * percent[0] * self.shape[0] as f64;
        self.center[1] += self.step_size * percent[1] * self.shape[1] as f64;
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

    pub fn create_pixels(&self) -> Vec<[u8; 3]> {
        let mut points: Vec<[f64; 2]> = Vec::new();
        for y in -self.shape[1] / 2..self.shape[1] / 2 {
            for x in -self.shape[0] / 2..self.shape[0] / 2 {
                let point = [self.center[0] + self.step_size * x as f64,
                             self.center[1] + self.step_size * y as f64];
                points.push(point); 
            }
        }
        points.par_iter()
            .map(|p| self.colorize_pixel(p))
            .collect()
    }

    fn colorize_pixel(&self, point: &[f64; 2]) -> [u8; 3] {
        match check_mandelbrot(point, self.depth) {
            Some(d) => {
                //hsv_to_rgb([d as f32 / self.depth as f32, 1., 1.])
                [(d as f32 / self.depth as f32 * 0xFF as f32) as u8, 0, 0xFF]
            },
            None => [0, 0, 0] 
        }
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
            shape: [800, 600],
            step_size: 1. / 800.,
            center: [0.5, 0.],
            depth: 20
        }
    }
}
