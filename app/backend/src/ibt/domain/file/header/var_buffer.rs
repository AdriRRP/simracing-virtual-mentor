use crate::ibt::domain::file::macros::num_from_le;

pub const VAR_BUFFER_BYTES_SIZE: usize = 16;

#[derive(PartialEq, Debug)]
pub struct VarBuffer {
    /// Used to detect changes in data
    /// Original type: i32 (4 byte integer)
    pub tick_count: u32,
    /// Offset from header
    /// Original type: i32 (4 byte integer)
    pub offset: u64,
    // Raw source contains HERE an 8 bytes padding for 16 bytes alignment
}

impl TryFrom<&[u8; VAR_BUFFER_BYTES_SIZE]> for VarBuffer {
    type Error = Error;

    fn try_from(bytes: &[u8; VAR_BUFFER_BYTES_SIZE]) -> Result<Self, Self::Error> {
        Ok(VarBuffer {
            tick_count: num_from_le!(&bytes, 0, 4, i32, Error, TickCount, u32),
            offset: num_from_le!(&bytes, 4, 8, i32, Error, Offset, u64),
        })
    }
}

/// Errors that can be returned from [`VarBuffer::try_from`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Var Buffer error extracting `tick_count`: {0}")]
    TickCount(String),
    #[error("Var Buffer error extracting `offset`: {0}")]
    Offset(String),
    #[error("Error trying to extract VarBuffer from Stream: {0}")]
    FromStream(String),
}
