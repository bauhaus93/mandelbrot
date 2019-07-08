#[macro_use]
extern crate log;
extern crate num;
#[macro_use]
extern crate rayon;
extern crate palette;
extern crate image;
extern crate histogram;
extern crate rand;

pub mod mandelbrot;
pub mod color_bucket;
pub mod mandelbrot_error;
mod snapshot;

pub use self::mandelbrot::Mandelbrot;
pub use self::color_bucket::ColorBucket;
pub use self::mandelbrot_error::MandelbrotError;
use self::snapshot::snapshot;
