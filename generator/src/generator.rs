use rand::{ Rng, SeedableRng };
use rand::rngs::StdRng;
use chrono::Local;

use mandelbrot_core::Mandelbrot;
use crate::GeneratorError;

pub struct Generator {
    snapshot_size: [i32; 2],
    entropy_threshold: f32,
    mandelbrot: Mandelbrot,
    rng: StdRng
}


impl Generator {
    pub fn new(snapshot_size: [i32; 2], entropy_threshold: f32) -> Result<Generator, GeneratorError> {
        let mut mandelbrot = Mandelbrot::default();
        mandelbrot.set_depth(400);
        let generator = Self {
            snapshot_size: snapshot_size,
            entropy_threshold: entropy_threshold,
            mandelbrot: mandelbrot,
            rng: StdRng::from_entropy()
        };

        Ok(generator)
    }


    pub fn run(&mut self) -> Result<(), GeneratorError> {
        loop {
            self.cycle()?;
        }
    }

    fn cycle(&mut self) -> Result<(), GeneratorError> {
        let entropy = self.randomize_mandelbrot();
        if entropy > self.entropy_threshold {
            info!("Entropy threshold reached, taking snapshot ({}x{})...",
                self.snapshot_size[0], self.snapshot_size[1]);
            let name = format!("{}", Local::now().format("%Y%m%d_%H%M%S"));
            let center = self.mandelbrot.get_center();
            info!("center = {}/{}, step_size = {}, depth = {}, entropy = {}, file = '{}'",
                center[0], center[1],
                self.mandelbrot.get_step_size(),
                self.mandelbrot.get_depth(),
                entropy,
                name);
            self.mandelbrot.snapshot(&name, self.snapshot_size)?;
            info!("Snapshot finished!");
        }
        Ok(())
    }

    fn randomize_mandelbrot(&mut self) -> f32 {
        let pos = self.get_random_pos();
        let step_size = self.get_random_step_size();
        let bucket_count = self.get_random_bucket_count();
        self.mandelbrot.set_center(pos);
        self.mandelbrot.set_step_size(step_size);
        self.mandelbrot.randomize_continuos_color_ranged(bucket_count as usize);
        let entropy = self.mandelbrot.estimate_entropy(self.snapshot_size);
        entropy
    }

    fn get_random_pos(&mut self) -> [f64; 2] {
        [self.rng.gen_range(-2., 2.),
         self.rng.gen_range(-2., 2.)]
    }
    fn get_random_step_size(&mut self) -> f64 {
        self.rng.gen_range(1e-14, 1e-4)
    }
    fn get_random_depth(&mut self) -> u32 {
        self.rng.gen_range(250, 750)
    }
    fn get_random_bucket_count(&mut self) -> u32 {
        self.rng.gen_range(100, 500)
    }
}
