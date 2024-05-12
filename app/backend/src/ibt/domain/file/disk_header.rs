use crate::ibt::domain::file::from_reader::FixedSize;
use crate::ibt::domain::file::macros::num_from_le;

use std::io::{Read, Seek};

pub const DISK_HEADER_BYTES_SIZE: usize = 32;

#[derive(PartialEq, Debug)]
pub struct DiskHeader {
    /// Original type: i64 (8 byte integer)
    pub start_date: u64, // Unix Time
    pub start_time: f64,
    pub end_time: f64,
    /// Original type: i32 (4 byte integer)
    pub lap_count: u32,
    /// Original type: i32 (4 byte integer)
    pub record_count: u32,
}

impl<ReadSeek> FixedSize<ReadSeek, Error, DISK_HEADER_BYTES_SIZE> for DiskHeader where
    ReadSeek: Read + Seek
{
}

impl TryFrom<&[u8; DISK_HEADER_BYTES_SIZE]> for DiskHeader {
    type Error = Error;

    fn try_from(bytes: &[u8; DISK_HEADER_BYTES_SIZE]) -> Result<Self, Self::Error> {
        Ok(DiskHeader {
            start_date: num_from_le!(&bytes, 0, 8, i64, Error, StartDate, u64),
            start_time: num_from_le!(&bytes, 8, 16, f64, Error, StartTime),
            end_time: num_from_le!(&bytes, 16, 24, f64, Error, EndTime),
            lap_count: num_from_le!(&bytes, 24, 28, i32, Error, LapCount, u32),
            record_count: num_from_le!(&bytes, 28, 32, i32, Error, RecordCount, u32),
        })
    }
}

/// Errors that can be returned from [`DiskHeader::try_from`].
#[derive(PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Disk Header error extracting `start_date`: {0}")]
    StartDate(String),
    #[error("Disk Header error extracting `start_time`: {0}")]
    StartTime(String),
    #[error("Disk Header error extracting `end_time`: {0}")]
    EndTime(String),
    #[error("Disk Header error extracting `lap_count`: {0}")]
    LapCount(String),
    #[error("Disk Header error extracting `record_count`: {0}")]
    RecordCount(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_bytes() -> [u8; DISK_HEADER_BYTES_SIZE] {
        [
            119, 207, 88, 101, 0, 0, 0, 0, 154, 169, 170, 122, 119, 119, 87, 64, 135, 234, 59, 34,
            162, 72, 164, 64, 28, 0, 0, 0, 130, 74, 2, 0,
        ]
    }

    fn expected_disk_header() -> DiskHeader {
        DiskHeader {
            start_date: 1700319095,
            start_time: 93.86666742960224,
            end_time: 2596.3166674350537,
            lap_count: 28,
            record_count: 150146,
        }
    }

    #[test]
    fn try_from_u8_slice_ok() {
        let result = DiskHeader::try_from(&test_bytes());
        let expected_result = Ok(expected_disk_header());
        assert_eq!(result, expected_result)
    }
}
