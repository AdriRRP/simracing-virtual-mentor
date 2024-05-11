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
    pub fn from_reader<ReadSeek: Read + Seek>(
        reader: &mut ReadSeek,
        var_header: &VarHeader,
        offset: u64,
        var_block_size: u64,
    ) -> Result<Metric, from_reader::Error> {
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

        Ok(Metric {
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
    use crate::ibt::domain::file::var_header::var_type::VarType;
    use super::*;

    //fn test_var_header() -> VarHeader {
    //    VarHeader {
    //        var_type: VarType::Char,
    //        offset: 0,
    //        count: 0,
    //        count_as_time: 0,
    //        name: [],
    //        description: [],
    //        unit: [],
    //    }
    //}

    //#[test]
    //fn

}
