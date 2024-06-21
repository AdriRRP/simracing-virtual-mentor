use std::fmt;
use std::fmt::{Debug, Display};

pub enum Error {
    NoSuchType(String),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NoSuchType(t_1), Self::NoSuchType(t_2)) => t_1 == t_2,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoSuchType(t) => write!(f, "No such filter type `{t}`."),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
