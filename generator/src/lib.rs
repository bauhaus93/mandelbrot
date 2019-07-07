#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;
extern crate rand;

extern crate mandelbrot_core;

pub mod generator;
pub mod generator_error;

pub use self::generator::Generator;
pub use self::generator_error::GeneratorError;
