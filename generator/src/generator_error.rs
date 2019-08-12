use std::error::Error;
use std::fmt;
use std::io;

use mandelbrot_core::MandelbrotError;

#[derive(Debug)]
pub enum GeneratorError {
    Io(io::Error),
    Mandelbrot(MandelbrotError),
}

impl From<io::Error> for GeneratorError {
    fn from(err: io::Error) -> Self {
        GeneratorError::Io(err)
    }
}

impl From<MandelbrotError> for GeneratorError {
    fn from(err: MandelbrotError) -> Self {
        GeneratorError::Mandelbrot(err)
    }
}
impl Error for GeneratorError {
    fn description(&self) -> &str {
        match *self {
            GeneratorError::Io(_) => "io",
            GeneratorError::Mandelbrot(_) => "mandelbrot",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            GeneratorError::Io(ref err) => Some(err),
            GeneratorError::Mandelbrot(ref err) => Some(err),
        }
    }
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GeneratorError::Io(ref err) => write!(f, "{}/{}", self.description(), err),
            GeneratorError::Mandelbrot(ref err) => write!(f, "{}/{}", self.description(), err),
        }
    }
}
