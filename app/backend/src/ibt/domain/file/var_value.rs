pub mod primitive;

use crate::ibt::domain::file::var_value::primitive::Primitive;

#[derive(Debug, Clone)]
pub enum VarValue {
    Single(Primitive),
    Array(Vec<Primitive>),
}

impl From<Primitive> for VarValue {
    fn from(value: Primitive) -> Self {
        Self::Single(value)
    }
}

impl From<Vec<Primitive>> for VarValue {
    fn from(values: Vec<Primitive>) -> Self {
        if values.len() == 1 {
            Self::Single(values[0].clone())
        } else {
            Self::Array(values)
        }
    }
}
