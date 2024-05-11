pub mod status;
pub mod var_buffer;

use crate::ibt::domain::file::from_reader::FromReaderFixedSize;
use crate::ibt::domain::file::header::status::Status;
use crate::ibt::domain::file::header::var_buffer::{VarBuffer, VAR_BUFFER_BYTES_SIZE};
use crate::ibt::domain::file::macros::num_from_le;

use std::io::{Read, Seek};

pub const HEADER_BYTES_SIZE: usize = 112;
const MAX_NUMBER_OF_BUFFERS: usize = 4;

#[derive(PartialEq, Debug)]
pub struct Header {
    /// Version of telemetry headers
    /// Original type: i32 (4 byte integer)
    pub version: i32,
    /// 1 if connected
    /// Original type: i32 (4 byte integer)
    pub status: Status,
    /// How often the game refreshes its information, usually measured in hertz
    /// Original type: i32 (4 byte integer)
    pub tick_rate: u32,

    /// Session information, updated periodically

    /// Incremented when session info changes
    /// Original type: i32 (4 byte integer)
    pub session_info_update: i32,
    /// Length in bytes of session info string
    /// Original type: i32 (4 byte integer)
    pub session_info_length: usize,
    /// Session info, encoded in YAML format
    /// Original type: i32 (4 byte integer)
    pub session_info_offset: u64,

    /// State data, output at tickRate

    /// length of array pointed to by var_header_offset
    /// Original type: i32 (4 byte integer)
    pub num_vars: usize,
    /// Offset to [VarHeader; num_vars] array
    /// describes the variables received in VarBuffer
    /// Original type: i32 (4 byte integer)
    pub var_header_offset: u64,

    /// Number of buffers to use
    /// Original type: i32 (4 byte integer)
    pub num_buf: u32,
    /// Size of each buffer
    /// Original type: i32 (4 byte integer)
    pub buf_len: u32,

    // Raw source contains HERE an 8 bytes padding for 16 bytes alignment

    // Up to this point, 48 bytes are taken up
    /// Buffers of data being written to (4 * 16 = 64 bytes)
    pub var_buffers: [VarBuffer; MAX_NUMBER_OF_BUFFERS],
}

impl TryFrom<&[u8; HEADER_BYTES_SIZE]> for Header {
    type Error = Error;

    fn try_from(bytes: &[u8; HEADER_BYTES_SIZE]) -> Result<Self, Self::Error> {
        Ok(Header {
            version: num_from_le!(bytes, 0, 4, i32, Error, Version),
            status: Status::from(num_from_le!(bytes, 4, 8, i32, Error, Status)),
            tick_rate: num_from_le!(bytes, 8, 12, i32, Error, TickRate, u32),
            session_info_update: num_from_le!(bytes, 12, 16, i32, Error, SessionInfoUpdate),
            session_info_length: num_from_le!(bytes, 16, 20, i32, Error, SessionInfoLength, usize),
            session_info_offset: num_from_le!(bytes, 20, 24, i32, Error, SessionInfoOffset, u64),
            num_vars: num_from_le!(bytes, 24, 28, i32, Error, NumVars, usize),
            var_header_offset: num_from_le!(bytes, 28, 32, i32, Error, VarHeaderOffset, u64),
            num_buf: num_from_le!(bytes, 32, 36, i32, Error, NumBuf, u32),
            buf_len: num_from_le!(bytes, 36, 40, i32, Error, BufLen, u32),
            var_buffers: {
                let mut buffer: [u8; VAR_BUFFER_BYTES_SIZE] = [0u8; VAR_BUFFER_BYTES_SIZE];

                let mut try_from_and_map_error = |start, end| {
                    buffer.copy_from_slice(&bytes[start..end]);
                    VarBuffer::try_from(&buffer.clone())
                        .map_err(|e| Error::VarBuffers(format!("{e}")))
                };

                [
                    try_from_and_map_error(48, 64)?,
                    try_from_and_map_error(64, 80)?,
                    try_from_and_map_error(80, 96)?,
                    try_from_and_map_error(96, 112)?,
                ]
            },
        })
    }
}

impl<ReadSeek> FromReaderFixedSize<ReadSeek, Error, HEADER_BYTES_SIZE> for Header where
    ReadSeek: Read + Seek
{
}

/// Errors that can be returned from [`Header::try_from`].
#[derive(PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Header error extracting `version`: {0}")]
    Version(String),
    #[error("Header error extracting `status`: {0}")]
    Status(String),
    #[error("Header error extracting `tick_rate`: {0}")]
    TickRate(String),
    #[error("Header error extracting `session_info_update`: {0}")]
    SessionInfoUpdate(String),
    #[error("Header error extracting `session_info_offset`: {0}")]
    SessionInfoOffset(String),
    #[error("Header error extracting `session_info_length`: {0}")]
    SessionInfoLength(String),
    #[error("Header error extracting `num_vars`: {0}")]
    NumVars(String),
    #[error("Header error extracting `var_header_offset`: {0}")]
    VarHeaderOffset(String),
    #[error("Header error extracting `num_buf`: {0}")]
    NumBuf(String),
    #[error("Header error extracting `buf_len`: {0}")]
    BufLen(String),
    #[error("Header error extracting `var_buffers`: {0}")]
    VarBuffers(String),
}

#[cfg(test)]
mod tests {
    use crate::ibt::domain::file::from_reader;
    use std::io::Cursor;

    use super::*;

    fn test_bytes() -> [u8; HEADER_BYTES_SIZE] {
        [
            2, 0, 0, 0, 1, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 34, 49, 0, 0, 16, 158, 0, 0, 24, 1, 0,
            0, 144, 0, 0, 0, 1, 0, 0, 0, 67, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 253, 15, 0, 0, 50,
            207, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ]
    }

    fn expected_header() -> Header {
        Header {
            version: 2,
            status: Status::Connected,
            tick_rate: 60,
            session_info_update: 0,
            session_info_length: 12578,
            session_info_offset: 40464,
            num_vars: 280,
            var_header_offset: 144,
            num_buf: 1,
            buf_len: 1091,
            var_buffers: [
                VarBuffer {
                    tick_count: 4093,
                    offset: 53042,
                },
                VarBuffer {
                    tick_count: 0,
                    offset: 0,
                },
                VarBuffer {
                    tick_count: 0,
                    offset: 0,
                },
                VarBuffer {
                    tick_count: 0,
                    offset: 0,
                },
            ],
        }
    }

    #[test]
    fn try_from_u8_slice_ok() {
        let result = Header::try_from(&test_bytes());
        let expected_result = Ok(expected_header());
        assert_eq!(result, expected_result)
    }

    #[test]
    fn from_reader_ok() {
        let test_bytes = test_bytes();
        let mut cursor = Cursor::new(&test_bytes);
        let result = Header::from_reader(&mut cursor, 0);
        let expected_result = Ok(expected_header());
        assert_eq!(result, expected_result)
    }

    #[test]
    fn from_reader_lower_than_header_size_ko() {
        let test_bytes: [u8; 100] = [0u8; 100];
        let mut cursor = Cursor::new(&test_bytes);
        let result = Header::from_reader(&mut cursor, 0);
        let expected_result = Err(from_reader::Error::Reading(
            "failed to fill whole buffer".to_string(),
        ));
        assert_eq!(result, expected_result)
    }

    #[test]
    fn from_reader_invalid_u32_ko() {
        let mut test_bytes: [u8; 112] = [0; 112];
        let invalid_bytes: [u8; 4] = [0xFF; 4];
        let start_index = 8;
        test_bytes[start_index..start_index + invalid_bytes.len()].copy_from_slice(&invalid_bytes);
        let mut cursor = Cursor::new(&test_bytes);
        let result = Header::from_reader(&mut cursor, 0);
        let expected_result = Err(from_reader::Error::Reading(
            "Header error extracting `tick_rate`: \
            -1 of type i32 cannot be converted converted to type u32: \
            out of range integral type conversion attempted"
                .to_string(),
        ));
        assert_eq!(result, expected_result)
    }

    #[test]
    fn from_reader_invalid_usize_ko() {
        let mut test_bytes: [u8; 112] = [0; 112];
        let invalid_bytes: [u8; 4] = [0xFF; 4];
        let start_index = 16;
        test_bytes[start_index..start_index + invalid_bytes.len()].copy_from_slice(&invalid_bytes);
        let mut cursor = Cursor::new(&test_bytes);
        let result = Header::from_reader(&mut cursor, 0);
        let expected_result = Err(from_reader::Error::Reading(
            "Header error extracting `session_info_length`: \
            -1 of type i32 cannot be converted converted to type usize: \
            out of range integral type conversion attempted"
                .to_string(),
        ));
        assert_eq!(result, expected_result)
    }

    #[test]
    fn from_reader_invalid_u64_ko() {
        let mut test_bytes: [u8; 112] = [0; 112];
        let invalid_bytes: [u8; 4] = [0xFF; 4];
        let start_index = 20;
        test_bytes[start_index..start_index + invalid_bytes.len()].copy_from_slice(&invalid_bytes);
        let mut cursor = Cursor::new(&test_bytes);
        let result = Header::from_reader(&mut cursor, 0);
        let expected_result = Err(from_reader::Error::Reading(
            "Header error extracting `session_info_offset`: \
            -1 of type i32 cannot be converted converted to type u64: \
            out of range integral type conversion attempted"
                .to_string(),
        ));
        assert_eq!(result, expected_result)
    }
}
