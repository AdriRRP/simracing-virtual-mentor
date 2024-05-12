pub mod primitive;

use crate::ibt::domain::file::from_reader;
use crate::ibt::domain::file::var_header::var_type::VarType;
use crate::ibt::domain::file::var_value::primitive::Primitive;

use std::io::{Read, Seek, SeekFrom};

#[derive(PartialEq, Debug, Clone)]
pub enum VarValue {
    Single(Primitive),
    Array(Vec<Primitive>),
}

impl VarValue {
    pub fn try_read_next<ReadSeek: Read + Seek>(
        reader: &mut ReadSeek,
        var_type: &VarType,
        var_count: u64,
        offset: u64,
    ) -> Result<Option<Self>, from_reader::Error> {
        // Option implements FromIterator, so you can move the Option outside and iterators will
        // take care of the rest (including stopping iteration if None is found).
        let opt_primitives_result: Result<Option<Vec<Primitive>>, from_reader::Error> = (0
            ..var_count)
            .map(|i| {
                let offset = offset + i * (var_type.byte_size() as u64);
                let mut buffer = vec![0u8; var_type.byte_size()];

                reader
                    .seek(SeekFrom::Start(offset))
                    .map_err(|e| from_reader::Error::Reading(format!("{e}")))?;

                let bytes_read = reader
                    .read(&mut buffer)
                    .map_err(|e| from_reader::Error::Reading(format!("{e}")))?;

                if bytes_read == 0 {
                    return Ok(None);
                } else if bytes_read != var_type.byte_size() {
                    return Err(from_reader::Error::Reading(format!(
                        "{} bytes are needed to read the {} type but only {} could be read",
                        var_type.byte_size(),
                        var_type,
                        bytes_read
                    )));
                }

                Ok(Some(Primitive::try_from((var_type, buffer)).map_err(
                    |e| from_reader::Error::Reading(format!("{e}")),
                )?))
            })
            .collect();

        opt_primitives_result.map(|opt| opt.map(|v| Self::from(v)))
    }
}

impl From<Primitive> for VarValue {
    fn from(value: Primitive) -> Self {
        Self::Single(value)
    }
}

impl From<Vec<Primitive>> for VarValue {
    fn from(values: Vec<Primitive>) -> Self {
        if values.len() == 1 {
            Self::Single(values[0].clone())
        } else {
            Self::Array(values)
        }
    }
}
