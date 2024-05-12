pub mod disk_header;
pub mod from_reader;
pub mod header;
pub mod macros;
pub mod metric;
pub mod metrics;
pub mod session_info;
pub mod var_filter;
pub mod var_header;
pub mod var_headers;
pub mod var_value;

use crate::ibt::domain::file::disk_header::DiskHeader;
use crate::ibt::domain::file::from_reader::{FixedSize, VariableSize};
use crate::ibt::domain::file::header::{Header, HEADER_BYTES_SIZE};
use crate::ibt::domain::file::metrics::Metrics;
use crate::ibt::domain::file::session_info::SessionInfo;

use crate::ibt::domain::file::var_filter::VarFilter;
use std::fmt::Debug;
use std::io::{Read, Seek};

#[derive(PartialEq, Debug)]
pub struct File {
    pub header: Header,
    pub disk_header: DiskHeader,
    pub session_info: SessionInfo,
    pub metrics: Metrics,
}

impl File {
    /// # Errors
    ///
    /// Will return `Err` if `Header::from_reader`, `DiskHeader::from_reader`,
    /// `SessionInfo::from_reader` or `Metrics::from_reader` fails
    #[allow(clippy::similar_names)]
    pub fn from_reader(
        reader: &mut (impl Read + Seek),
        filter: &Option<VarFilter>,
    ) -> Result<Self, Error> {
        let header = Header::from_reader(reader, 0).map_err(|e| Error::Header(format!("{e}")))?;

        let disk_header = DiskHeader::from_reader(reader, HEADER_BYTES_SIZE as u64)
            .map_err(|e| Error::Header(format!("{e}")))?;

        let session_info = SessionInfo::from_reader(
            reader,
            header.session_info_offset,
            header.session_info_length,
        )
        .map_err(|e| Error::SessionInfo(format!("{e}")))?;

        let metrics = Metrics::from_reader(reader, &header, filter)
            .map_err(|e| Error::Metrics(format!("{e}")))?;

        // Return complete file
        Ok(Self {
            header,
            disk_header,
            session_info,
            metrics,
        })
    }
}

/// Errors that can be returned from [`Header::try_from`].
#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error("File error extracting `header`: {0}")]
    Header(String),
    #[error("File error extracting `disk_header`: {0}")]
    DiskHeader(String),
    #[error("File error extracting `session_info`: {0}")]
    SessionInfo(String),
    #[error("File error extracting `metrics`: {0}")]
    Metrics(String),
}
