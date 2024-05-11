use crate::ibt::domain::file::macros::num_from_le;
use crate::ibt::domain::file::var_header::var_type::VarType;

#[derive(PartialEq, Debug, Copy, Clone)]
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

impl TryFrom<(&VarType, Vec<u8>)> for Primitive {
    type Error = Error;

    fn try_from(value: (&VarType, Vec<u8>)) -> Result<Self, Self::Error> {
        let (var_type, bytes) = value;
        match (var_type, bytes.len()) {
            (VarType::Bool, 1) => Ok(Primitive::Bool(bytes[0] != 0u8)),
            (VarType::Char, 1) => Ok(Primitive::Char(char::from(bytes[0]))),
            (VarType::Int, 4) => Ok(Self::Int(num_from_le!(bytes, 0, 4, i32, Error, Int))),
            (VarType::BitField, 4) => Ok(Self::BitField(num_from_le!(
                bytes, 0, 4, u32, Error, BitField
            ))),
            (VarType::Float, 4) => Ok(Self::Float(num_from_le!(bytes, 0, 4, f32, Error, Float))),
            (VarType::Double, 8) => Ok(Self::Double(num_from_le!(bytes, 0, 8, f64, Error, Double))),
            (t, s) => Err(Error::IncompatibleNumberOfBytes(format!(
                "Size of {t} isn't {s} bytes"
            ))),
        }
    }
}

#[derive(PartialEq, Debug, thiserror::Error)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn char_ok() {
        let var_type = VarType::Char;
        let vec = vec![97u8];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Ok(Primitive::Char('a'));
        assert_eq!(current, expected)
    }

    #[test]
    fn char_ko() {
        let var_type = VarType::Char;
        let vec = vec![97u8, 0u8];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Err(Error::IncompatibleNumberOfBytes(
            "Size of Char  isn't 2 bytes".to_string(),
        ));
        assert_eq!(current, expected)
    }

    #[test]
    fn bool_ok() {
        let var_type = VarType::Bool;
        let vec = vec![0u8];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Ok(Primitive::Bool(false));
        assert_eq!(current, expected)
    }

    #[test]
    fn bool_ko() {
        let var_type = VarType::Bool;
        let vec = vec![0u8, 0u8];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Err(Error::IncompatibleNumberOfBytes(
            "Size of Bool isn't 2 bytes".to_string(),
        ));
        assert_eq!(current, expected)
    }

    #[test]
    fn int_ok() {
        let var_type = VarType::Int;
        let vec = vec![0u8; 4];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Ok(Primitive::Int(0));
        assert_eq!(current, expected)
    }

    #[test]
    fn int_ko() {
        let var_type = VarType::Int;
        let vec = vec![0u8, 0u8];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Err(Error::IncompatibleNumberOfBytes(
            "Size of Int isn't 2 bytes".to_string(),
        ));
        assert_eq!(current, expected)
    }

    #[test]
    fn bit_field_ok() {
        let var_type = VarType::BitField;
        let vec = vec![0u8; 4];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Ok(Primitive::BitField(0));
        assert_eq!(current, expected)
    }

    #[test]
    fn bit_field_ko() {
        let var_type = VarType::BitField;
        let vec = vec![0u8, 0u8];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Err(Error::IncompatibleNumberOfBytes(
            "Size of BitField isn't 2 bytes".to_string(),
        ));
        assert_eq!(current, expected)
    }

    #[test]
    fn float_ok() {
        let var_type = VarType::Float;
        let vec = vec![0u8; 4];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Ok(Primitive::Float(0f32));
        assert_eq!(current, expected)
    }

    #[test]
    fn float_ko() {
        let var_type = VarType::Float;
        let vec = vec![0u8, 0u8];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Err(Error::IncompatibleNumberOfBytes(
            "Size of Float isn't 2 bytes".to_string(),
        ));
        assert_eq!(current, expected)
    }

    #[test]
    fn double_ok() {
        let var_type = VarType::Double;
        let vec = vec![0u8; 8];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Ok(Primitive::Double(0f64));
        assert_eq!(current, expected)
    }

    #[test]
    fn double_ko() {
        let var_type = VarType::Double;
        let vec = vec![0u8, 0u8];
        let current = Primitive::try_from((&var_type, vec));
        let expected = Err(Error::IncompatibleNumberOfBytes(
            "Size of Double isn't 2 bytes".to_string(),
        ));
        assert_eq!(current, expected)
    }
}
