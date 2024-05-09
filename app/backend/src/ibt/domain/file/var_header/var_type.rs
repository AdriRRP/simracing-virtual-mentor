use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum VarType {
    // 1 byte
    Char = 0,
    Bool,
    // 4 byte
    Int,
    BitField,
    Float,
    // 8 bytes
    Double,
    //index, don't use
    ETCount,
}

impl VarType {
    pub fn byte_size(&self) -> usize {
        match self {
            Self::Char | Self::Bool => 1,
            Self::Int | Self::BitField | Self::Float => 4,
            Self::Double => 8,
            Self::ETCount => 0,
        }
    }
}

impl TryFrom<i32> for VarType {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VarType::Char),
            1 => Ok(VarType::Bool),
            2 => Ok(VarType::Int),
            3 => Ok(VarType::BitField),
            4 => Ok(VarType::Float),
            5 => Ok(VarType::Double),
            6 => Ok(VarType::ETCount),
            unknown => Err(Error::FromI32(format!("{unknown}"))),
        }
    }
}

impl fmt::Display for VarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Char => "Char ",
            Self::Bool => "Bool",
            Self::Int => "Int",
            Self::BitField => "BitField",
            Self::Float => "Float",
            Self::Double => "Double",
            Self::ETCount => "ETCount",
        };
        write!(f, "{msg}")
    }
}

pub enum Error {
    FromI32(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let common_msg = "Var Type error extracting";
        let error_msg = match self {
            Self::FromI32(msg) => format!("{common_msg} from i32: {msg}"),
        };
        write!(f, "{error_msg}")
    }
}
