use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ApplicationError {
    Allegro(String),
}

impl Error for ApplicationError {

    fn description(&self) -> &str {
        match *self {
            ApplicationError::Allegro(_) => "allegro",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApplicationError::Allegro(_) => None,
        }
    }
}       

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationError::Allegro(ref text) => write!(f, "{}: {}", self.description(), text),
        }
    }
}
