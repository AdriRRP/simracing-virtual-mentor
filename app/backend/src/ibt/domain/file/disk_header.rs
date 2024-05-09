use crate::ibt::domain::file::macros::num_from_le;

use crate::ibt::domain::file::header::HEADER_BYTES_SIZE;
use std::io::{Read, Seek, SeekFrom};

pub const DISK_HEADER_BYTES_SIZE: usize = 32;

#[derive(Debug)]
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

impl DiskHeader {
    pub fn from_stream(reader: &mut (impl Read + Seek)) -> Result<Self, Error> {
        let mut disk_header_buffer = [0u8; DISK_HEADER_BYTES_SIZE];

        reader
            .seek(SeekFrom::Start(HEADER_BYTES_SIZE as u64))
            .map_err(|e| Error::FromStream(format!("{e}")))?;

        reader
            .read_exact(&mut disk_header_buffer)
            .map_err(|e| Error::FromStream(format!("{e}")))?;

        DiskHeader::try_from(&disk_header_buffer)
    }
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
#[derive(Debug, thiserror::Error)]
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
    #[error("Error trying to extract Disk Header from Stream: {0}")]
    FromStream(String),
}
