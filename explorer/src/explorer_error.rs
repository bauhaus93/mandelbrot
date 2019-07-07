use std::fmt;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum ExplorerError {
    Allegro(String),
    Io(io::Error)
}

impl From<io::Error> for ExplorerError {
    fn from(err: io::Error) -> Self {
        ExplorerError::Io(err)
    }
}

impl Error for ExplorerError {

    fn description(&self) -> &str {
        match *self {
            ExplorerError::Allegro(_) => "allegro",
            ExplorerError::Io(_) => "io"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ExplorerError::Allegro(_) => None,
            ExplorerError::Io(ref err) => Some(err)
        }
    }
}       

impl fmt::Display for ExplorerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExplorerError::Allegro(ref text) => write!(f, "{}: {}", self.description(), text),
            ExplorerError::Io(ref err) => write!(f, "{}/{}", self.description(), err)
        }
    }
}
