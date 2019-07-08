use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use palette::{ Srgb, Hsv, Pixel };

#[derive(Clone, Copy)]
pub struct ColorBucket {
    color: [u8; 3]
}


impl ColorBucket {
    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> ColorBucket {
        let rgb_f: [f32; 3] = Srgb::from(Hsv::new(360. * hue, saturation, value)).into_raw();
        Self {
            color: [(255. * rgb_f[0]) as u8,
                    (255. * rgb_f[1]) as u8,
                    (255. * rgb_f[2]) as u8]
        }
    }

    pub fn create_continuos_list(start_hue: f32, bucket_count: usize) -> Vec<ColorBucket> {
        let hue_step = 1. / bucket_count as f32;
        let mut buckets = Vec::new();
        for i in 0..bucket_count {
            let hue = match start_hue + hue_step * i as f32 {
                hue if hue > 1. => hue - 1.,
                hue => hue
            };
            let b = ColorBucket::from_hsv(hue, 1., 1.);
            buckets.push(b);
        }
        buckets
    }

    pub fn create_random_continuos_list(bucket_count: usize) -> Vec<ColorBucket> {
        let start = thread_rng().gen_range(0., 1.);
        Self::create_continuos_list(start, bucket_count)
    }

    pub fn create_random_list(bucket_count: usize) -> Vec<ColorBucket> {
        let mut rng = thread_rng();
        let mut buckets = Vec::new();
        for _i in 0..bucket_count {
            let hue = rng.gen_range(0., 1.);
            let b = ColorBucket::from_hsv(hue, 1., 1.);
            buckets.push(b);
        }
        buckets
    }

    pub fn create_random_alternating_list(bucket_count: usize, color_count: usize) -> Vec<ColorBucket> {
        let mut buckets = Vec::new();
        let mut diff_buckets = Self::create_random_list(color_count);
        for i in 0..bucket_count {
            buckets.push(diff_buckets[i % color_count]);
        }
        buckets
    }

    pub fn get_color(&self) -> [u8; 3] {
        self.color
    }
}