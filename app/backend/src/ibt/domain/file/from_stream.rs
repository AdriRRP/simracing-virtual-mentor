use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

pub trait FromStream<ReadSeek, TryFromError, const SIZE: usize>
where
    ReadSeek: Read + Seek,
    TryFromError: From<Error>,
    Self: Sized + for<'a> TryFrom<&'a [u8; SIZE], Error = TryFromError>,
{
    fn from_stream(reader: &mut ReadSeek, offset: u64) -> Result<Self, TryFromError> {
        let mut buffer = [0u8; SIZE];

        reader
            .seek(SeekFrom::Start(offset))
            .map_err(|e| Error::FromStream(format!("{e}")))?;
        reader
            .read_exact(&mut buffer)
            .map_err(|e| Error::FromStream(format!("{e}")))?;

        Self::try_from(&buffer)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot extract data from stream: {0}")]
    FromStream(String),
}
