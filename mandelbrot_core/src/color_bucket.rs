use palette::{Hsv, Pixel, Srgb};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct ColorBucket {
    color: [u8; 3],
}

impl ColorBucket {
    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> ColorBucket {
        let rgb_f: [f32; 3] = Srgb::from(Hsv::new(360. * hue, saturation, value)).into_raw();
        Self {
            color: [
                (255. * rgb_f[0]) as u8,
                (255. * rgb_f[1]) as u8,
                (255. * rgb_f[2]) as u8,
            ],
        }
    }

    pub fn get_hue(&self) -> f32 {
        Hsv::from(Srgb::new(
            self.color[0] as f32 / 255.,
            self.color[1] as f32 / 255.,
            self.color[2] as f32 / 255.,
        ))
        .into_components()
        .0
        .to_positive_degrees()
            / 360.
    }

    pub fn random_bucket() -> ColorBucket {
        let hue = thread_rng().gen_range(0., 1.);
        ColorBucket::from_hsv(hue, 1., 1.)
    }

    pub fn next_bucket(&self, loop_depth: i32) -> ColorBucket {
        let hue = self.get_hue() + (1. / loop_depth as f32);
        ColorBucket::from_hsv(hue, 1., 1.)
    }

    pub fn get_color(&self) -> [u8; 3] {
        self.color
    }
}
