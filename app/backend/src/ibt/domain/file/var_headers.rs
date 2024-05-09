use crate::ibt::domain::file::header::Header;
use crate::ibt::domain::file::var_header::VarHeader;
use crate::ibt::domain::file::Error;
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct VarHeaders {
    var_headers: Vec<VarHeader>,
    index: usize,
}

impl VarHeaders {
    pub fn from_stream(reader: &mut (impl Read + Seek), header: &Header) -> Result<Self, Error> {
        let mut var_headers: Vec<VarHeader> = Vec::with_capacity(header.num_vars);

        (0..header.num_vars).try_for_each(|i| -> Result<(), Error> {
            let position: u64 = i
                .try_into()
                .map_err(|e| Error::VarHeaders(format!("{e}")))?;

            let var_header_offset: u64 =
                header.var_header_offset + (position * header.var_header_offset);

            let var_header = VarHeader::from_stream(reader, var_header_offset)
                .map_err(|e| Error::VarHeaders(format!("{e}")))?;

            var_headers.push(var_header);

            Ok(())
        })?;

        Ok(Self {
            var_headers,
            index: 0,
        })
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.var_headers.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.var_headers.len()
    }
}

impl Iterator for VarHeaders {
    type Item = VarHeader;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.var_headers.get(self.index);
        match next {
            Some(e) => {
                self.index += 1;
                Some(e.clone())
            }
            None => None,
        }
    }
}

impl PartialEq for VarHeaders {
    fn eq(&self, other: &Self) -> bool {
        self.var_headers.len() == other.var_headers.len() && {
            let zipped_var_headers: Vec<(&VarHeader, &VarHeader)> = self
                .var_headers
                .iter()
                .zip(other.var_headers.iter())
                .collect();
            zipped_var_headers.iter().all(|(a, b)| a == b)
        }
    }
}
