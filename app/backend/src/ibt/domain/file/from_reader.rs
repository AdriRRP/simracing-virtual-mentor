use std::fmt::{Debug, Display};
use std::io::{Read, Seek, SeekFrom};

pub trait FromReaderFixedSize<ReadSeek, TryFromError, const SIZE: usize>
where
    ReadSeek: Read + Seek,
    TryFromError: Display + Debug,
    Self: Sized + for<'a> TryFrom<&'a [u8; SIZE], Error = TryFromError>,
{
    fn from_reader(reader: &mut ReadSeek, offset: u64) -> Result<Self, Error> {
        let mut buffer = [0u8; SIZE];

        reader
            .seek(SeekFrom::Start(offset))
            .map_err(|e| Error::Reading(format!("{e}")))?;
        reader
            .read_exact(&mut buffer)
            .map_err(|e| Error::Reading(format!("{e}")))?;

        Self::try_from(&buffer).map_err(|e| Error::Reading(format!("{}", e)))
    }
}

pub trait FromReaderVarSize<ReadSeek, TryFromError>
where
    ReadSeek: Read + Seek,
    TryFromError: Display + Debug,
    Self: for<'a> TryFrom<&'a Vec<u8>, Error = TryFromError>,
{
    fn from_reader(reader: &mut ReadSeek, offset: u64, size: usize) -> Result<Self, Error> {
        let mut buffer = vec![0u8; size];

        reader
            .seek(SeekFrom::Start(offset))
            .map_err(|e| Error::Reading(format!("{e}")))?;
        reader
            .read_exact(&mut buffer)
            .map_err(|e| Error::Reading(format!("{e}")))?;

        Self::try_from(&buffer).map_err(|e| Error::Reading(format!("{}", e)))
    }
}

#[derive(PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot extract data from reader: {0}")]
    Reading(String),
}
