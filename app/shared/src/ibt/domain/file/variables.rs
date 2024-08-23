use crate::ibt::domain::file::from_reader;
use crate::ibt::domain::file::header::Header;
use crate::ibt::domain::file::var_filter::VarFilter;
use crate::ibt::domain::file::var_headers::VarHeaders;
use crate::ibt::domain::file::variable::Variable;

use std::io::{Read, Seek};
use std::ops::Deref;

/// Represents a collection of variables.
#[derive(PartialEq, Clone, Debug)]
pub struct Variables {
    variables: Vec<Variable>,
}

impl Variables {
    /// Constructs a `Variables` instance from a reader, given the header information and an optional variable filter.
    ///
    /// # Errors
    ///
    /// Returns an error if the maximum by tick count `header.var_buffers` can't be retrieved,
    /// or if constructing individual variable using `Variable::from_reader` fails.
    #[allow(clippy::similar_names)]
    pub fn from_reader<ReadSeek: Read + Seek>(
        reader: &mut ReadSeek,
        header: &Header,
        filter: &Option<VarFilter>,
    ) -> Result<Self, from_reader::Error> {
        // Headers of all variables
        let mut var_headers = VarHeaders::from_reader(reader, header)?;

        // Size of a sample of values of all headers
        let var_block_size = var_headers
            .iter()
            .fold(0usize, |acc, e| acc + e.var_type.byte_size() * e.count)
            as u64;

        // Filter headers
        if let Some(var_filter) = filter {
            var_headers.retain(|var_header| var_filter.allow(var_header));
        }

        // Collect variables from var headers
        let metrics_result: Result<Vec<Variable>, from_reader::Error> = var_headers
            .iter()
            .map(|var_header| {
                // Pick the buffer with the highest tick_count
                let current_buffer = header
                    .var_buffers
                    .iter()
                    .max_by_key(|b| b.tick_count)
                    .ok_or_else(|| {
                        from_reader::Error::Reading(
                            "Can't get the buffer with the highest tick_count".to_string(),
                        )
                    })?;
                Variable::from_reader(reader, var_header, current_buffer.offset, var_block_size)
            })
            .collect();

        metrics_result.map(|metrics| Self { variables: metrics })
    }
}

impl Deref for Variables {
    type Target = Vec<Variable>;

    fn deref(&self) -> &Self::Target {
        &self.variables
    }
}
