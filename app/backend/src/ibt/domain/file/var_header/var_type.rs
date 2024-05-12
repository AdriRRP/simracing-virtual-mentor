use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
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
    #[must_use]
    pub const fn byte_size(&self) -> usize {
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
            0 => Ok(Self::Char),
            1 => Ok(Self::Bool),
            2 => Ok(Self::Int),
            3 => Ok(Self::BitField),
            4 => Ok(Self::Float),
            5 => Ok(Self::Double),
            6 => Ok(Self::ETCount),
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

#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Var Type error extracting from i32: {0}")]
    FromI32(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_ok() {
        let current = VarType::try_from(0);
        let expected = Ok(VarType::Char);
        assert_eq!(current, expected)
    }

    #[test]
    fn char_size_ok() {
        let current = VarType::Char.byte_size();
        let expected = 1;
        assert_eq!(current, expected)
    }

    #[test]
    fn bool_ok() {
        let current = VarType::try_from(1);
        let expected = Ok(VarType::Bool);
        assert_eq!(current, expected)
    }

    #[test]
    fn bool_size_ok() {
        let current = VarType::Bool.byte_size();
        let expected = 1;
        assert_eq!(current, expected)
    }

    #[test]
    fn int_ok() {
        let current = VarType::try_from(2);
        let expected = Ok(VarType::Int);
        assert_eq!(current, expected)
    }

    #[test]
    fn int_size_ok() {
        let current = VarType::Int.byte_size();
        let expected = 4;
        assert_eq!(current, expected)
    }

    #[test]
    fn bit_field_ok() {
        let current = VarType::try_from(3);
        let expected = Ok(VarType::BitField);
        assert_eq!(current, expected)
    }

    #[test]
    fn bit_field_int_ok() {
        let current = VarType::BitField.byte_size();
        let expected = 4;
        assert_eq!(current, expected)
    }

    #[test]
    fn float_ok() {
        let current = VarType::try_from(4);
        let expected = Ok(VarType::Float);
        assert_eq!(current, expected)
    }

    #[test]
    fn float_size_ok() {
        let current = VarType::Float.byte_size();
        let expected = 4;
        assert_eq!(current, expected)
    }

    #[test]
    fn double_ok() {
        let current = VarType::try_from(5);
        let expected = Ok(VarType::Double);
        assert_eq!(current, expected)
    }

    #[test]
    fn double_size_ok() {
        let current = VarType::Double.byte_size();
        let expected = 8;
        assert_eq!(current, expected)
    }

    #[test]
    fn et_count_ok() {
        let current = VarType::try_from(6);
        let expected = Ok(VarType::ETCount);
        assert_eq!(current, expected)
    }

    #[test]
    fn et_count_size_ok() {
        let current = VarType::ETCount.byte_size();
        let expected = 0;
        assert_eq!(current, expected)
    }

    #[test]
    fn unknown_ko() {
        let current = VarType::try_from(-1);
        let expected = Err(Error::FromI32("-1".to_string()));
        assert_eq!(current, expected)
    }
}
