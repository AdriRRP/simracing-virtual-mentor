pub mod var_type;

use crate::ibt::domain::file::from_reader::FixedSize;
use crate::ibt::domain::file::macros::{num_from_le, str_from_le};
use crate::ibt::domain::file::var_header::var_type::VarType;

use std::io::{Read, Seek};

pub const VAR_HEADER_BYTES_SIZE: usize = 144;
pub const MAX_STRING_BYTES_SIZE: usize = 32;
pub const MAX_DESCRIPTION_BYTES_SIZE: usize = 64;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct VarHeader {
    /// Data type of the variable value
    /// Original type: i32 (4 byte integer)
    pub var_type: VarType,
    /// Offset from start of buffer row
    /// Original type: i32 (4 byte integer)
    pub offset: u64,
    /// Number of samples containing this variable
    /// In case it is greater than 1, it will be an array
    /// Therefore the size in bytes of the variable will be Count * varType.byte_size()
    /// Original type: i32 (4 byte integer)
    pub count: usize,

    /// 1 byte
    pub count_as_time: i8,

    // Raw source contains HERE an 3 bytes padding for 16 bytes alignment
    /// 32 x 1 bytes
    pub(crate) name: [char; MAX_STRING_BYTES_SIZE],
    /// 64 x 1 bytes
    pub(crate) description: [char; MAX_DESCRIPTION_BYTES_SIZE],
    /// Something like "kg/m^2"
    /// 32 x 1 bytes
    pub(crate) unit: [char; MAX_STRING_BYTES_SIZE],
}

impl VarHeader {
    #[must_use]
    pub fn name(&self) -> String {
        self.name
            .into_iter()
            .filter(|c| c.ne(&char::from(0)))
            .collect::<String>()
    }

    #[must_use]
    pub fn description(&self) -> String {
        self.description
            .into_iter()
            .filter(|c| c.ne(&char::from(0)))
            .collect::<String>()
    }

    #[must_use]
    pub fn unit(&self) -> String {
        self.unit
            .into_iter()
            .filter(|c| c.ne(&char::from(0)))
            .collect::<String>()
    }
}

impl TryFrom<&[u8; VAR_HEADER_BYTES_SIZE]> for VarHeader {
    type Error = Error;

    fn try_from(bytes: &[u8; VAR_HEADER_BYTES_SIZE]) -> Result<Self, Self::Error> {
        Ok(Self {
            var_type: num_from_le!(&bytes, 0, 4, i32, Error, VarTypeExtract, VarType),
            offset: num_from_le!(&bytes, 4, 8, i32, Error, Offset, u64),
            count: num_from_le!(&bytes, 8, 12, i32, Error, Count, usize),
            count_as_time: num_from_le!(&bytes, 12, 13, i8, Error, CountAsTime),
            // padding here, 16 byte align (3 bytes)
            name: str_from_le!(&bytes, 16, MAX_STRING_BYTES_SIZE, Error, Name),
            description: str_from_le!(&bytes, 48, MAX_DESCRIPTION_BYTES_SIZE, Error, Description),
            unit: str_from_le!(&bytes, 112, MAX_STRING_BYTES_SIZE, Error, Unit),
        })
    }
}

impl<ReadSeek> FixedSize<ReadSeek, Error, VAR_HEADER_BYTES_SIZE> for VarHeader where
    ReadSeek: Read + Seek
{
}

/// Errors that can be returned from [`VarHeader::try_from`].
#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Disk Header error extracting `var_type_extract`: {0}")]
    VarTypeExtract(String),
    #[error("Disk Header error extracting `offset`: {0}")]
    Offset(String),
    #[error("Disk Header error extracting `count`: {0}")]
    Count(String),
    #[error("Disk Header error extracting `count_as_time`: {0}")]
    CountAsTime(String),
    #[error("Disk Header error extracting `name`: {0}")]
    Name(String),
    #[error("Disk Header error extracting `description`: {0}")]
    Description(String),
    #[error("Disk Header error extracting `unit`: {0}")]
    Unit(String),
    #[error("Error trying to extract VarHeader from Stream: {0}")]
    FromStream(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_bytes() -> [u8; VAR_HEADER_BYTES_SIZE] {
        [
            5, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 83, 101, 115, 115, 105, 111, 110, 84,
            105, 109, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 101,
            99, 111, 110, 100, 115, 32, 115, 105, 110, 99, 101, 32, 115, 101, 115, 115, 105, 111,
            110, 32, 115, 116, 97, 114, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    }

    fn expected_var_header() -> VarHeader {
        VarHeader {
            var_type: VarType::Double,
            offset: 0,
            count: 1,
            count_as_time: 0,
            name: [
                'S', 'e', 's', 's', 'i', 'o', 'n', 'T', 'i', 'm', 'e', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0',
            ],
            description: [
                'S', 'e', 'c', 'o', 'n', 'd', 's', ' ', 's', 'i', 'n', 'c', 'e', ' ', 's', 'e',
                's', 's', 'i', 'o', 'n', ' ', 's', 't', 'a', 'r', 't', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0',
            ],
            unit: [
                's', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0',
            ],
        }
    }

    #[test]
    fn try_from_u8_slice_ok() {
        let result = VarHeader::try_from(&test_bytes());
        let expected_result = Ok(expected_var_header());
        assert_eq!(result, expected_result);
    }
    #[test]
    fn name_ok() {
        let current_var_header = VarHeader::try_from(&test_bytes()).unwrap();
        assert_eq!(current_var_header.name(), "SessionTime".to_string());
    }

    #[test]
    fn description_ok() {
        let current_var_header = VarHeader::try_from(&test_bytes()).unwrap();
        assert_eq!(
            current_var_header.description(),
            "Seconds since session start".to_string()
        );
    }
    #[test]
    fn unit_ok() {
        let current_var_header = VarHeader::try_from(&test_bytes()).unwrap();
        assert_eq!(current_var_header.unit(), "s".to_string());
    }
}
