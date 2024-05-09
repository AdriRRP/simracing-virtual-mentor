use crate::ibt::domain::file::macros::num_from_le;
use crate::ibt::domain::file::var_header::var_type::VarType;

#[derive(Debug, Copy, Clone)]
pub enum Primitive {
    // 1 byte
    Char(char),
    Bool(bool),
    // 4 byte
    Int(i32),
    BitField(u32),
    Float(f32),
    // 8 bytes
    Double(f64),
}

impl Primitive {
    pub fn as_char(&self) -> Option<char> {
        match self {
            Self::Char(c) => Some(c.clone()),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(b.clone()),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Self::Int(i) => Some(i.clone()),
            _ => None,
        }
    }

    pub fn as_bit_field(&self) -> Option<u32> {
        match self {
            Self::BitField(bf) => Some(bf.clone()),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            Self::Float(f) => Some(f.clone()),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        match self {
            Self::Double(d) => Some(d.clone()),
            _ => None,
        }
    }
}

impl TryFrom<(&VarType, [u8; 1])> for Primitive {
    type Error = Error;

    fn try_from(value: (&VarType, [u8; 1])) -> Result<Self, Self::Error> {
        let (var_type, bytes) = value;
        match var_type {
            VarType::Char => Ok(Primitive::Char(char::from(bytes[0]))),
            VarType::Bool => Ok(Primitive::Bool(bytes[0] != 0u8)),
            VarType::Int | VarType::BitField | VarType::Float => {
                Err(Error::IncompatibleNumberOfBytes(format!(
                    "{var_type} need 4 bytes, but only 1 byte was provided"
                )))
            }
            VarType::Double => Err(Error::IncompatibleNumberOfBytes(format!(
                "{var_type} need 8 bytes, but only 1 byte was provided"
            ))),
            _ => Err(Error::IncompatibleNumberOfBytes(format!(
                "Illegal type for VarValue: {var_type}"
            ))),
        }
    }
}

impl TryFrom<(&VarType, [u8; 4])> for Primitive {
    type Error = Error;

    fn try_from(value: (&VarType, [u8; 4])) -> Result<Self, Self::Error> {
        let (var_type, bytes) = value;
        match var_type {
            VarType::Char | VarType::Bool => Err(Error::IncompatibleNumberOfBytes(format!(
                "{var_type} need 1 bytes, but 4 bytes was provided"
            ))),
            VarType::Int => Ok(Self::Int(num_from_le!(bytes, 0, 4, i32, Error, Int))),
            VarType::BitField => Ok(Self::BitField(num_from_le!(
                bytes, 0, 4, u32, Error, BitField
            ))),
            VarType::Float => Ok(Self::Float(num_from_le!(bytes, 0, 4, f32, Error, Float))),
            VarType::Double => Err(Error::IncompatibleNumberOfBytes(format!(
                "{var_type} need 8 bytes, but only 4 byte was provided"
            ))),
            _ => Err(Error::IncompatibleNumberOfBytes(format!(
                "Illegal type for VarValue: {var_type}"
            ))),
        }
    }
}

impl TryFrom<(&VarType, [u8; 8])> for Primitive {
    type Error = Error;

    fn try_from(value: (&VarType, [u8; 8])) -> Result<Self, Self::Error> {
        let (var_type, bytes) = value;
        match var_type {
            VarType::Char | VarType::Bool => Err(Error::IncompatibleNumberOfBytes(format!(
                "{var_type} need 1 bytes, but 8 bytes was provided"
            ))),
            VarType::Int | VarType::BitField | VarType::Float => {
                Err(Error::IncompatibleNumberOfBytes(format!(
                    "{var_type} need 4 bytes, but 8 bytes was provided"
                )))
            }
            VarType::Double => Ok(Self::Double(num_from_le!(bytes, 0, 8, f64, Error, Double))),
            _ => Err(Error::IncompatibleNumberOfBytes(format!(
                "Illegal type for VarValue: {var_type}"
            ))),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Incompatible number of bytes provided: {0}")]
    IncompatibleNumberOfBytes(String),
    #[error("Cannot extract Int: {0}")]
    Int(String),
    #[error("Cannot extract BitField: {0}")]
    BitField(String),
    #[error("Cannot extract Float: {0}")]
    Float(String),
    #[error("Cannot extract Double: {0}")]
    Double(String),
}
