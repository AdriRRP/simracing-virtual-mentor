use crate::ibt::domain::file::from_reader::Error;
use crate::ibt::domain::file::from_reader::FixedSize;
use crate::ibt::domain::file::header::Header;
use crate::ibt::domain::file::var_header::{VarHeader, VAR_HEADER_BYTES_SIZE};

use std::io::{Read, Seek};
use std::ops::{Deref, DerefMut};

#[derive(PartialEq, Debug)]
pub struct VarHeaders {
    var_headers: Vec<VarHeader>,
}

impl VarHeaders {
    /// # Errors
    ///
    /// Will return `Err` if `VarHeader::from_reader` fails
    #[allow(clippy::similar_names)]
    pub fn from_reader(reader: &mut (impl Read + Seek), header: &Header) -> Result<Self, Error> {
        // Result implements FromIterator, so you can move the Result outside and iterators will
        // take care of the rest (including stopping iteration if an error is found).
        let var_headers_result: Result<Vec<VarHeader>, Error> = (0..header.num_vars)
            .map(|i| -> Result<VarHeader, Error> {
                let current_offset: u64 =
                    header.var_header_offset + ((i * VAR_HEADER_BYTES_SIZE) as u64);
                VarHeader::from_reader(reader, current_offset)
            })
            .collect();

        var_headers_result.map(|var_headers| Self { var_headers })
    }
}

impl Deref for VarHeaders {
    type Target = Vec<VarHeader>;

    fn deref(&self) -> &Self::Target {
        &self.var_headers
    }
}

impl DerefMut for VarHeaders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.var_headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ibt::domain::file::header::status::Status;
    use crate::ibt::domain::file::header::var_buffer::VarBuffer;
    use crate::ibt::domain::file::var_header::var_type::VarType;
    use std::io::Cursor;

    fn test_bytes() -> Vec<u8> {
        vec![
            5, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 83, 101, 115, 115, 105, 111, 110, 84,
            105, 109, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 101,
            99, 111, 110, 100, 115, 32, 115, 105, 110, 99, 101, 32, 115, 101, 115, 115, 105, 111,
            110, 32, 115, 116, 97, 114, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 99, 2,
            0, 0, 6, 0, 0, 0, 1, 0, 0, 0, 83, 116, 101, 101, 114, 105, 110, 103, 87, 104, 101, 101,
            108, 84, 111, 114, 113, 117, 101, 95, 83, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 117,
            116, 112, 117, 116, 32, 116, 111, 114, 113, 117, 101, 32, 111, 110, 32, 115, 116, 101,
            101, 114, 105, 110, 103, 32, 115, 104, 97, 102, 116, 32, 97, 116, 32, 51, 54, 48, 32,
            72, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 42,
            109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ]
    }

    fn test_header() -> Header {
        Header {
            version: 0,
            status: Status::Connected,
            tick_rate: 0,
            session_info_update: 0,
            session_info_length: 0,
            session_info_offset: 0,
            num_vars: 2,
            var_header_offset: 0,
            num_buf: 0,
            buf_len: 0,
            var_buffers: [
                VarBuffer {
                    tick_count: 0,
                    offset: 0,
                },
                VarBuffer {
                    tick_count: 0,
                    offset: 0,
                },
                VarBuffer {
                    tick_count: 0,
                    offset: 0,
                },
                VarBuffer {
                    tick_count: 0,
                    offset: 0,
                },
            ],
        }
    }

    fn expected_var_headers() -> VarHeaders {
        VarHeaders {
            var_headers: vec![
                VarHeader {
                    var_type: VarType::Double,
                    offset: 0,
                    count: 1,
                    count_as_time: 0,
                    name: [
                        'S', 'e', 's', 's', 'i', 'o', 'n', 'T', 'i', 'm', 'e', '\0', '\0', '\0',
                        '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                        '\0', '\0', '\0', '\0', '\0', '\0',
                    ],
                    description: [
                        'S', 'e', 'c', 'o', 'n', 'd', 's', ' ', 's', 'i', 'n', 'c', 'e', ' ', 's',
                        'e', 's', 's', 'i', 'o', 'n', ' ', 's', 't', 'a', 'r', 't', '\0', '\0',
                        '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                        '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                        '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                    ],
                    unit: [
                        's', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                        '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                        '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                    ],
                },
                VarHeader {
                    var_type: VarType::Float,
                    offset: 611,
                    count: 6, // Each variable value is an array of 6 floats
                    count_as_time: 1,
                    name: [
                        'S', 't', 'e', 'e', 'r', 'i', 'n', 'g', 'W', 'h', 'e', 'e', 'l', 'T', 'o',
                        'r', 'q', 'u', 'e', '_', 'S', 'T', '\0', '\0', '\0', '\0', '\0', '\0',
                        '\0', '\0', '\0', '\0',
                    ],
                    description: [
                        'O', 'u', 't', 'p', 'u', 't', ' ', 't', 'o', 'r', 'q', 'u', 'e', ' ', 'o',
                        'n', ' ', 's', 't', 'e', 'e', 'r', 'i', 'n', 'g', ' ', 's', 'h', 'a', 'f',
                        't', ' ', 'a', 't', ' ', '3', '6', '0', ' ', 'H', 'z', '\0', '\0', '\0',
                        '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                        '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                    ],
                    unit: [
                        'N', '*', 'm', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                        '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                        '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                    ],
                },
            ],
        }
    }

    #[test]
    fn try_from_reader_ok() {
        let test_bytes = test_bytes();
        let mut cursor = Cursor::new(&test_bytes);
        let result = VarHeaders::from_reader(&mut cursor, &test_header());
        let expected_result = Ok(expected_var_headers());
        assert_eq!(result, expected_result);
    }
}
