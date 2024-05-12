use crate::ibt::domain::file::from_reader;
use crate::ibt::domain::file::var_header::VarHeader;
use crate::ibt::domain::file::var_value::VarValue;

use std::io::{Read, Seek};
use std::ops::Deref;

#[derive(PartialEq, Debug)]
pub struct Metric {
    pub var_header: VarHeader,
    pub var_values: Vec<VarValue>,
}

impl Metric {
    /// # Errors
    ///
    /// Will return `Err` if `VarValue::try_read_next` fails
    pub fn from_reader<ReadSeek: Read + Seek>(
        reader: &mut ReadSeek,
        var_header: &VarHeader,
        offset: u64,
        var_block_size: u64,
    ) -> Result<Self, from_reader::Error> {
        let mut var_values: Vec<VarValue> = Vec::new();
        let mut offset = offset;

        while let Some(value) = VarValue::try_read_next(
            reader,
            &var_header.var_type,
            var_header.count as u64,
            offset + var_header.offset,
        )? {
            var_values.push(value);
            offset += var_block_size;
        }

        Ok(Self {
            var_header: var_header.clone(),
            var_values,
        })
    }
}

impl Deref for Metric {
    type Target = Vec<VarValue>;

    fn deref(&self) -> &Self::Target {
        &self.var_values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ibt::domain::file::var_header::var_type::VarType;
    use crate::ibt::domain::file::var_value::primitive::Primitive;
    use std::io::Cursor;

    fn test_bytes() -> Vec<u8> {
        vec!['a' as u8]
    }

    fn test_var_header() -> VarHeader {
        VarHeader {
            var_type: VarType::Char,
            offset: 0,
            count: 1,
            count_as_time: 0,
            name: [
                'S', 'e', 's', 's', 'i', 'o', 'n', 'T', 'i', 'm', 'e', '!', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0',
            ],
            description: [
                'S', 'e', 'c', 'o', 'n', 'd', 's', ' ', 's', 'i', 'n', 'c', 'e', ' ', 's', 'e',
                's', 's', 'i', 'o', 'n', ' ', 's', 't', 'a', 'r', 't', '!', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0',
            ],
            unit: [
                's', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0',
            ],
        }
    }

    #[test]
    fn read_ok() {
        let var_header = test_var_header();
        let test_bytes = test_bytes();
        let mut cursor = Cursor::new(&test_bytes);
        let result = Metric::from_reader(
            &mut cursor,
            &var_header,
            0,
            VarType::Char.byte_size() as u64,
        );
        let expected = Ok(Metric {
            var_header,
            var_values: vec![VarValue::Single(Primitive::Char('a'))],
        });
        assert_eq!(result, expected)
    }

    #[test]
    fn read_ko() {
        let mut var_header = test_var_header();
        var_header.var_type = VarType::Double;
        let test_bytes = test_bytes();
        let mut cursor = Cursor::new(&test_bytes);
        let result = Metric::from_reader(
            &mut cursor,
            &var_header,
            0,
            VarType::Char.byte_size() as u64,
        );
        let expected = Err(from_reader::Error::Reading(
            "8 bytes are needed to read the Double type but only 1 could be read".to_string(),
        ));
        assert_eq!(result, expected)
    }
}
