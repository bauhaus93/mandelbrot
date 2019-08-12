#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate allegro;
extern crate allegro_primitives;
extern crate mandelbrot_core;

pub mod explorer;
pub mod explorer_error;

pub use self::explorer::Explorer;
pub use self::explorer_error::ExplorerError;
