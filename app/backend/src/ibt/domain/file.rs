pub mod disk_header;
pub mod from_stream;
pub mod header;
pub mod macros;
pub mod metric;
pub mod session_info;
pub mod var_header;
pub mod var_headers;
pub mod var_value;

use crate::ibt::domain::file::disk_header::DiskHeader;
use crate::ibt::domain::file::header::Header;
use crate::ibt::domain::file::session_info::SessionInfo;
use crate::ibt::domain::file::var_headers::VarHeaders;
use crate::ibt::domain::file::var_value::{primitive::Primitive, VarValue};

use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};
use crate::ibt::domain::file::from_stream::FromStream;

#[derive(Debug)]
pub struct File {
    pub header: Header,
    pub disk_header: DiskHeader,
    pub session_info: SessionInfo,
    pub var_values: Vec<VarValue>,
}

impl File {
    fn read_var_values(
        reader: &mut (impl Read + Seek),
        var_header_offset: u64,
        var_headers: &mut VarHeaders,
    ) -> Result<Vec<VarValue>, Error> {
        let mut var_values: Vec<VarValue> = Vec::new();

        var_headers.try_for_each(|var_header| -> Result<(), Error> {
            let mut var_value_buffer_1 = [0u8; 1];
            let mut var_value_buffer_4 = [0u8; 4];
            let mut var_value_buffer_8 = [0u8; 8];

            let mut primitive_values: Vec<Primitive> = Vec::new();

            (0..var_header.count).try_for_each(|i| -> Result<(), Error> {
                let elem_offset = u64::try_from(i * var_header.var_type.byte_size())
                    .map_err(|e| Error::VarHeaders(format!("{e}")))?;
                let offset = var_header_offset + var_header.offset + elem_offset;

                reader
                    .seek(SeekFrom::Start(offset))
                    .map_err(|e| Error::VarValues(format!("{e}")))?;

                let primitive = match var_header.var_type.byte_size() {
                    1 => {
                        reader
                            .read_exact(&mut var_value_buffer_1)
                            .map_err(|e| Error::VarValues(format!("{e}")))?;
                        Primitive::try_from((&var_header.var_type, var_value_buffer_1))
                            .map_err(|e| Error::VarValues(format!("{e}")))
                    }
                    4 => {
                        reader
                            .read_exact(&mut var_value_buffer_4)
                            .map_err(|e| Error::VarValues(format!("{e}")))?;
                        Primitive::try_from((&var_header.var_type, var_value_buffer_4))
                            .map_err(|e| Error::VarValues(format!("{e}")))
                    }
                    8 => {
                        reader
                            .read_exact(&mut var_value_buffer_8)
                            .map_err(|e| Error::VarValues(format!("{e}")))?;
                        Primitive::try_from((&var_header.var_type, var_value_buffer_8))
                            .map_err(|e| Error::VarValues(format!("{e}")))
                    }
                    n => Err(Error::VarValues(format!(
                        "Invalid var_value number of bytes {n}"
                    ))),
                }?;

                primitive_values.push(primitive);

                Ok(())
            })?;

            var_values.push(VarValue::from(primitive_values));

            Ok(())
        })?;

        Ok(var_values)
    }

    pub fn from_stream(reader: &mut (impl Read + Seek)) -> Result<Self, Error> {
        let header = Header::from_stream(reader, 0).map_err(|e| Error::Header(format!("{e}")))?;
        let disk_header =
            DiskHeader::from_stream(reader).map_err(|e| Error::Header(format!("{e}")))?;
        let session_info = SessionInfo::from_stream(
            reader,
            header.session_info_offset,
            header.session_info_length,
        )
        .map_err(|e| Error::SessionInfo(format!("{e}")))?;
        let mut var_headers = VarHeaders::from_stream(reader, &header)?;
        let var_values = Self::read_var_values(reader, header.var_header_offset, &mut var_headers)?;

        // Return complete file
        Ok(File {
            header,
            disk_header,
            session_info,
            var_values,
        })
    }
}

/// Errors that can be returned from [`Header::try_from`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("File error extracting `header`: {0}")]
    Header(String),
    #[error("File error extracting `disk_header`: {0}")]
    DiskHeader(String),
    #[error("File error extracting `session_info`: {0}")]
    SessionInfo(String),
    #[error("File error extracting `var_headers`: {0}")]
    VarHeaders(String),
    #[error("File error extracting `var_values`: {0}")]
    VarValues(String),
}
