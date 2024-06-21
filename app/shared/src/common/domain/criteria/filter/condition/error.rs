use std::fmt;
use std::fmt::{Debug, Display};

pub enum Error {
    NoSuchOperator(String),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NoSuchOperator(op_1), Self::NoSuchOperator(op_2)) => op_1 == op_2,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoSuchOperator(op) => write!(f, "No such condition `{op}`."),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
