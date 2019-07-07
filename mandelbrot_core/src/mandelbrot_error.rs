use std::fmt;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum MandelbrotError {
    Io(io::Error)
}

impl From<io::Error> for MandelbrotError {
    fn from(err: io::Error) -> Self {
        MandelbrotError::Io(err)
    }
}

impl Error for MandelbrotError {

    fn description(&self) -> &str {
        match *self {
            MandelbrotError::Io(_) => "io"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            MandelbrotError::Io(ref err) => Some(err)
        }
    }
}       

impl fmt::Display for MandelbrotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MandelbrotError::Io(ref err) => write!(f, "{}/{}", self.description(), err)
        }
    }
}
