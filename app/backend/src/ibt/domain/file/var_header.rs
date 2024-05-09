pub mod var_type;

use crate::ibt::domain::file::macros::{num_from_le, str_from_le};
use crate::ibt::domain::file::var_header::var_type::VarType;

use std::io::{Read, Seek, SeekFrom};

pub const VAR_HEADER_BYTES_SIZE: usize = 144;
pub const MAX_STRING_BYTES_SIZE: usize = 32;
pub const MAX_DESCRIPTION_BYTES_SIZE: usize = 64;

#[derive(Clone, Debug, PartialEq)]
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

    pub count_as_time: i8,
    name: [char; MAX_STRING_BYTES_SIZE],
    description: [char; MAX_DESCRIPTION_BYTES_SIZE],
    /// Something like "kg/m^2"
    unit: [char; MAX_STRING_BYTES_SIZE],
}

impl VarHeader {
    pub fn name(&self) -> String {
        self.name
            .iter()
            .collect::<String>()
            .trim_matches(char::from(0))
            .parse()
            .unwrap()
    }
    pub fn description(&self) -> String {
        self.description
            .iter()
            .collect::<String>()
            .trim_matches(char::from(0))
            .parse()
            .unwrap()
    }
    pub fn unit(&self) -> String {
        self.unit
            .iter()
            .collect::<String>()
            .trim_matches(char::from(0))
            .parse()
            .unwrap()
    }

    pub fn from_stream(reader: &mut (impl Read + Seek), offset: u64) -> Result<Self, Error> {
        let mut header_buffer = [0u8; VAR_HEADER_BYTES_SIZE];

        reader
            .seek(SeekFrom::Start(offset))
            .map_err(|e| Error::FromStream(format!("{e}")))?;

        reader
            .read_exact(&mut header_buffer)
            .map_err(|e| Error::FromStream(format!("{e}")))?;

        Self::try_from(&header_buffer)
    }
}

impl TryFrom<&[u8; VAR_HEADER_BYTES_SIZE]> for VarHeader {
    type Error = Error;

    fn try_from(bytes: &[u8; VAR_HEADER_BYTES_SIZE]) -> Result<Self, Self::Error> {
        Ok(VarHeader {
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

/// Errors that can be returned from [`VarHeader::try_from`].
#[derive(Debug, thiserror::Error)]
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
